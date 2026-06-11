use tauri::State;

use crate::errors::AppResult;
use crate::kernel::protocol;
use crate::models::debug::DebugFrame;
use crate::state::app_state::AppState;

/// Return captured IPC frames for the debug diagnostic page.
/// Newest frames first; limited to the ring buffer capacity (~200).
#[tauri::command]
pub fn gui_debug_frames(_state: State<'_, AppState>) -> AppResult<Vec<DebugFrame>> {
    Ok(protocol::debug_frames_snapshot())
}
