//! Single persistent IPC connection with request/event multiplexing.
//!
//! Per the kernel docs, a single named-pipe / unix-socket connection
//! should carry all traffic: subscribe first, then multiplex query,
//! command, and ping frames on the same stream.  Response frames have
//! an `ok` key; event frames have a `schema_id` key.  Heartbeat lines
//! (`:\n`) are skipped.
//!
//! This module replaces the old per-request `send_json_line_request`
//! model which opened a new pipe for every call, exhausting pipe
//! instances (error 231) under concurrent load.
//!
//! ## Implementation note
//!
//! On Windows named pipes, `ReadFile` blocks until data arrives.  To
//! avoid deadlocking writes while the reader thread waits for the next
//! frame, we open **two handles to the same pipe**: one dedicated to
//! the reader thread, one serialised by a mutex for writes.  Both
//! handles refer to the same kernel pipe server instance, so the
//! logical connection is still a single multiplexed stream.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::Value;
use tokio::sync::oneshot;

use crate::errors::{AppError, AppResult};
use crate::models::core::CoreEndpoint;

use super::transport;

type PendingRequests = Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value, AppError>>>>>;
type EventSender = tokio::sync::broadcast::Sender<Value>;

static NEXT_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) struct MultiplexedIpc {
    /// Write handle — serialised by a mutex to avoid interleaved frames.
    writer: Mutex<Box<dyn transport::ReadWrite>>,
    /// Dedicated read handle for the reader thread — never locked by writes.
    reader: Mutex<Option<Box<dyn transport::ReadWrite>>>,
    /// Pending requests waiting for a response, keyed by request `id`.
    pending: PendingRequests,
    /// Event broadcast channel — populated by the reader thread.
    events: EventSender,
    /// Connection endpoint for error messages.
    endpoint: CoreEndpoint,
    /// Set to true once subscribe has been sent on this connection.
    subscribed: Mutex<bool>,
}

impl MultiplexedIpc {
    /// Open a persistent IPC connection to `endpoint`.
    /// Opens two handles: one for writing, one for the reader thread.
    pub fn connect(endpoint: CoreEndpoint, timeout: Duration) -> AppResult<Arc<Self>> {
        let writer_stream = transport::connect_raw(&endpoint, timeout)?;
        let reader_stream = transport::connect_raw(&endpoint, timeout)?;
        let pending: PendingRequests = Arc::new(Mutex::new(HashMap::new()));
        let events: EventSender = tokio::sync::broadcast::channel(256).0;

        let conn = Arc::new(Self {
            writer: Mutex::new(writer_stream),
            reader: Mutex::new(Some(reader_stream)),
            pending: pending.clone(),
            events: events.clone(),
            endpoint: endpoint.clone(),
            subscribed: Mutex::new(false),
        });

        // Spawn the reader thread
        let conn_reader = Arc::clone(&conn);
        let endpoint_reader = endpoint.clone();
        std::thread::spawn(move || {
            conn_reader.read_loop(endpoint_reader, pending, events);
        });

        Ok(conn)
    }

    /// Send a request frame and wait for the matching response.
    pub fn send_request(self: &Arc<Self>, frame: Value, timeout: Duration) -> AppResult<Value> {
        let id = NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        let mut frame = frame;
        if let Some(obj) = frame.as_object_mut() {
            if !obj.contains_key("id") {
                obj.insert("id".to_string(), Value::Number(id.into()));
            }
        }

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self
                .pending
                .lock()
                .map_err(|_| AppError::internal("pending lock poisoned"))?;
            pending.insert(id, tx);
        }

        // Write the frame (uses the dedicated write handle, not the reader's)
        {
            let frame_bytes = transport::serialize_frame(&frame)?;
            let mut writer = self
                .writer
                .lock()
                .map_err(|_| AppError::internal("writer lock poisoned"))?;
            writer
                .write_all(&frame_bytes)
                .map_err(|e| AppError::from_io("failed to write IPC frame", &self.endpoint, e))?;
            writer
                .flush()
                .map_err(|e| AppError::from_io("failed to flush IPC frame", &self.endpoint, e))?;
        }

        // Wait for response
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                let result = handle.block_on(tokio::time::timeout(timeout, rx));
                match result {
                    Ok(Ok(response)) => response,
                    Ok(Err(_)) => {
                        let _ = self.pending.lock().map(|mut p| p.remove(&id));
                        Err(AppError::internal("IPC connection closed"))
                    }
                    Err(_) => {
                        let _ = self.pending.lock().map(|mut p| p.remove(&id));
                        Err(AppError::internal(format!(
                            "IPC request timed out after {}ms",
                            timeout.as_millis()
                        )))
                    }
                }
            }
            Err(_) => match rx.blocking_recv() {
                Ok(response) => response,
                Err(_) => {
                    let _ = self.pending.lock().map(|mut p| p.remove(&id));
                    Err(AppError::internal("IPC connection closed"))
                }
            },
        }
    }

    /// Subscribe to kernel events on this connection.
    pub fn subscribe(
        self: &Arc<Self>,
        event_types: Option<&[String]>,
    ) -> AppResult<tokio::sync::broadcast::Receiver<Value>> {
        let mut subscribed = self
            .subscribed
            .lock()
            .map_err(|_| AppError::internal("subscribed lock poisoned"))?;
        if *subscribed {
            return Ok(self.events.subscribe());
        }

        let frame = match event_types {
            Some(types) => serde_json::json!({ "type": "subscribe", "events": types }),
            None => serde_json::json!({ "type": "subscribe" }),
        };

        let confirmation = self.send_request(frame, Duration::from_secs(5))?;
        let ok = confirmation
            .as_object()
            .and_then(|obj| obj.get("ok"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !ok {
            return Err(AppError::core_response(confirmation));
        }

        *subscribed = true;
        Ok(self.events.subscribe())
    }

    /// Background reader loop — uses the dedicated read handle.
    fn read_loop(
        &self,
        endpoint: CoreEndpoint,
        pending: PendingRequests,
        events: EventSender,
    ) {
        // Take the reader handle — it lives exclusively in this thread
        let mut reader = {
            let mut guard = match self.reader.lock() {
                Ok(g) => g,
                Err(_) => return,
            };
            guard.take().expect("reader handle already consumed")
        };

        let mut buf = String::new();
        loop {
            buf.clear();
            match Self::read_json_line(&mut *reader, &mut buf, &endpoint) {
                Ok(value) => {
                    if value.as_object().map_or(false, |obj| obj.contains_key("ok")) {
                        // Response frame → resolve pending request by id
                        let id = value
                            .as_object()
                            .and_then(|obj| obj.get("id"))
                            .and_then(|v| v.as_u64());
                        let response: Result<Value, AppError> = if value
                            .as_object()
                            .and_then(|obj| obj.get("ok"))
                            .and_then(|v| v.as_bool())
                            == Some(true)
                        {
                            Ok(value)
                        } else {
                            Err(AppError::core_response(value))
                        };

                        if let Some(id) = id {
                            if let Ok(mut pending) = pending.lock() {
                                if let Some(tx) = pending.remove(&id) {
                                    let _ = tx.send(response);
                                }
                            }
                        }
                    } else if value
                        .as_object()
                        .map_or(false, |obj| obj.contains_key("schema_id"))
                    {
                        let _ = events.send(value);
                    }
                }
                Err(_) => break,
            }
        }
    }

    /// Read a single JSON-line from the stream.
    fn read_json_line(
        reader: &mut dyn Read,
        buf: &mut String,
        endpoint: &CoreEndpoint,
    ) -> AppResult<Value> {
        loop {
            buf.clear();
            let mut byte_buf = [0u8; 1];
            loop {
                match reader.read(&mut byte_buf) {
                    Ok(0) => return Err(AppError::connection_closed(endpoint)),
                    Ok(_) => {
                        if byte_buf[0] == b'\n' {
                            break;
                        }
                        buf.push(byte_buf[0] as char);
                    }
                    Err(e) => {
                        return Err(AppError::from_io(
                            "failed to read IPC frame",
                            endpoint,
                            e,
                        ));
                    }
                }
            }

            let line = buf.trim();
            if line.is_empty() || line.starts_with(':') {
                continue;
            }

            return serde_json::from_str::<Value>(line).map_err(|error| AppError {
                code: "internal",
                message: format!("failed to parse IPC frame: {error}"),
                details: Some(serde_json::json!({ "raw": line })),
            });
        }
    }
}

// ── Global connection singleton ─────────────────────────────────────

use std::sync::OnceLock;

static GLOBAL_IPC: OnceLock<Mutex<Option<Arc<MultiplexedIpc>>>> = OnceLock::new();

pub(crate) fn get_or_connect(
    endpoint: CoreEndpoint,
    timeout: Duration,
) -> AppResult<Arc<MultiplexedIpc>> {
    let cell = GLOBAL_IPC.get_or_init(|| Mutex::new(None));
    let mut guard = cell
        .lock()
        .map_err(|_| AppError::internal("global ipc lock poisoned"))?;

    if let Some(ref conn) = *guard {
        return Ok(Arc::clone(conn));
    }

    let conn = MultiplexedIpc::connect(endpoint, timeout)?;
    *guard = Some(Arc::clone(&conn));
    Ok(conn)
}

pub(crate) fn subscribe_events(
    endpoint: CoreEndpoint,
    event_types: Option<&[String]>,
) -> AppResult<tokio::sync::broadcast::Receiver<Value>> {
    let conn = get_or_connect(endpoint, Duration::from_secs(5))?;
    conn.subscribe(event_types)
}
