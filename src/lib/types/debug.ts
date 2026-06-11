/** A single IPC frame captured for the debug diagnostic page. */
export interface DebugFrame {
  /** Monotonic sequence number. */
  id: number;
  /** Unix ms timestamp. */
  atMs: number;
  /** "tx" (GUI → kernel) or "rx" (kernel → GUI). */
  direction: 'tx' | 'rx';
  /** "ping" | "query" | "command" | "subscribe" | "response" */
  frameType: string;
  /** JSON payload (may be truncated for large responses). */
  payload: unknown;
  /** Elapsed ms since the matching request (rx frames only). */
  elapsedMs?: number;
  /** Error string if the request failed. */
  error?: string;
}
