use crate::errors::{AppError, AppResult};
use crate::services::common::lock;
use crate::state::app_state::AppState;

pub const UI_MODE_LITE: &str = "lite";
pub const UI_MODE_PRO: &str = "pro";

pub fn current_ui_mode(state: &AppState) -> AppResult<String> {
    Ok(lock(state.app_config(), "app_config")?.ui.ui_mode.clone())
}

pub fn is_pro_mode(ui_mode: &str) -> bool {
    ui_mode.eq_ignore_ascii_case(UI_MODE_PRO)
}

pub fn require_pro_mode(state: &AppState, feature: &'static str) -> AppResult<()> {
    let ui_mode = current_ui_mode(state)?;
    if is_pro_mode(&ui_mode) {
        return Ok(());
    }

    Err(AppError::mode_restricted(feature, UI_MODE_PRO, ui_mode))
}
