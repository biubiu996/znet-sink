use tauri::State;

use crate::errors::AppResult;
use crate::models::gui_core::GuiSelfTestSnapshot;
use crate::services::gui_self_test;
use crate::state::app_state::AppState;

#[tauri::command]
pub async fn gui_self_test_snapshot(state: State<'_, AppState>) -> AppResult<GuiSelfTestSnapshot> {
    gui_self_test::snapshot(state).await
}
