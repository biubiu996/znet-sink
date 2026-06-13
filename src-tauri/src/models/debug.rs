use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// A captured IPC frame for the debug diagnostic page.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugFrame {
    /// Monotonic sequence number.
    pub id: u64,
    /// Unix ms timestamp when captured.
    pub at_ms: u64,
    /// "tx" (GUI → kernel) or "rx" (kernel → GUI).
    #[serde(rename = "direction")]
    pub direction: &'static str,
    /// "ping" | "query" | "command" | "subscribe" | "response" | "event" | "hb"
    pub frame_type: String,
    /// The JSON payload (may be truncated for large responses).
    pub payload: serde_json::Value,
    /// Elapsed ms since the matching request (response frames only).
    pub elapsed_ms: Option<u64>,
    /// Error string if the request failed.
    pub error: Option<String>,
}

/// Maximum number of frames retained in the ring buffer.
pub(crate) const DEBUG_RING_SIZE: usize = 200;

/// Global ring buffer for diagnostic IPC frame capture.
static DEBUG_FRAMES: std::sync::LazyLock<Mutex<Vec<DebugFrame>>> =
    std::sync::LazyLock::new(|| Mutex::new(Vec::with_capacity(DEBUG_RING_SIZE)));

static DEBUG_FRAME_ID: AtomicU64 = AtomicU64::new(0);

/// Push a frame into the ring buffer from anywhere in the crate.
pub(crate) fn push_debug_frame(frame: DebugFrame) {
    let mut frame = frame;
    frame.id = DEBUG_FRAME_ID.fetch_add(1, Ordering::Relaxed);
    if let Ok(mut frames) = DEBUG_FRAMES.lock() {
        if frames.len() >= DEBUG_RING_SIZE {
            frames.remove(0);
        }
        frames.push(frame);
    }
}

/// Snapshot all captured debug frames (newest first).
pub(crate) fn snapshot_debug_frames() -> Vec<DebugFrame> {
    DEBUG_FRAMES
        .lock()
        .map(|frames| {
            let mut v = frames.clone();
            v.reverse();
            v
        })
        .unwrap_or_default()
}

/// Clear all captured debug frames.
pub(crate) fn clear_debug_frames() {
    if let Ok(mut frames) = DEBUG_FRAMES.lock() {
        frames.clear();
    }
}
