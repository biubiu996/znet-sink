use tauri::State;

use crate::errors::AppResult;
use crate::kernel::protocol;
use crate::models::debug::DebugFrame;
use crate::state::app_state::AppState;

#[tauri::command]
pub fn gui_debug_frames(_state: State<'_, AppState>) -> AppResult<Vec<DebugFrame>> {
    Ok(protocol::debug_frames_snapshot())
}

#[tauri::command]
pub fn gui_debug_clear(_state: State<'_, AppState>) -> AppResult<()> {
    protocol::debug_clear();
    Ok(())
}
