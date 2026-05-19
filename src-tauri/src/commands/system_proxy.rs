use tauri::State;

use crate::errors::AppResult;
use crate::services::common::lock;
use crate::services::system_proxy::{self, SystemProxyStatus};
use crate::state::app_state::AppState;

#[tauri::command]
pub fn system_proxy_enable(state: State<'_, AppState>) -> AppResult<SystemProxyStatus> {
    let config = lock(state.app_config(), "app_config")?;
    system_proxy::enable(&config.local_proxy.host, config.local_proxy.port)
}

#[tauri::command]
pub fn system_proxy_disable() -> AppResult<SystemProxyStatus> {
    system_proxy::disable()
}

#[tauri::command]
pub fn system_proxy_status() -> AppResult<SystemProxyStatus> {
    system_proxy::status()
}
