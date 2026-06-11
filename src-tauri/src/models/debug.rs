use serde::Serialize;

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
    /// "ping" | "query" | "command" | "subscribe" | "response"
    pub frame_type: String,
    /// The JSON payload (may be truncatued for large responses).
    pub payload: serde_json::Value,
    /// Elapsed ms since the matching request (response frames only).
    pub elapsed_ms: Option<u64>,
    /// Error string if the request failed.
    pub error: Option<String>,
}

/// Maximum number of frames retained in the ring buffer.
pub(crate) const DEBUG_RING_SIZE: usize = 200;
