//! Effective-proxy coordinator.
//!
//! `reqwest` (and the Tauri updater plugin) only read `HTTPS_PROXY` /
//! `HTTP_PROXY` environment variables — they do NOT consult the OS system
//! proxy on Windows/macOS. So even when a proxy is available (another proxy
//! app set the OS proxy, or the kernel is running on its mixed-port), every
//! outbound request from the GUI process goes direct and fails when the
//! destination is unreachable directly.
//!
//! This module keeps the env vars in sync with the best-available proxy:
//!
//! ```text
//! effective = first available of:
//!   1. OS system proxy (whoever set it — the GUI or another app)
//!   2. kernel mixed-port (if the kernel is running)
//!   3. direct (no env vars)
//! ```
//!
//! When the GUI itself enables the system proxy it points it at the kernel's
//! mixed-port, so case 1 naturally covers the "GUI enabled system proxy"
//! scenario. Case 2 is what makes the kernel-only scenario (no system proxy
//! set) still work. Case 3 keeps the updater alive when neither is available,
//! so the GUI can always download a fresh kernel over a direct connection.
//!
//! `download_proxy_auto == false` short-circuits to "direct" to honour the
//! user's explicit opt-out.

use crate::errors::AppResult;
use crate::models::core_process::CoreProcessState;
use crate::services::{common, system_proxy};
use crate::state::app_state::AppState;

const PROXY_ENV_VARS: [&str; 4] = ["http_proxy", "https_proxy", "HTTP_PROXY", "HTTPS_PROXY"];

/// Compute the proxy URL the GUI process should use right now, or `None`
/// for a direct connection. See the module docs for the priority order.
pub fn resolve_effective_proxy(state: &AppState) -> Option<String> {
    let config = common::lock(state.app_config(), "app_config").ok()?;

    // User explicitly disabled "follow proxy" → always direct.
    if !config.core.download_proxy_auto {
        return None;
    }

    // 1. OS system proxy (set by the GUI or by another proxy app).
    if let Ok(os_status) = system_proxy::status() {
        if os_status.enabled && !os_status.host.is_empty() && os_status.port > 0 {
            return Some(format!("http://{}:{}", os_status.host, os_status.port));
        }
    }

    // 2. Kernel is running → reuse its mixed-port even when no OS proxy is set.
    // Read the cached state rather than probing — the watchdog keeps it
    // fresh, and we don't want every env-var update to do a network ping.
    let core_running = common::lock(state.core_process(), "core_process")
        .map(|p| p.status.state == CoreProcessState::Running)
        .unwrap_or(false);
    if core_running {
        let lp = &config.local_proxy;
        if !lp.host.is_empty() && lp.port > 0 {
            return Some(format!("http://{}:{}", lp.host, lp.port));
        }
    }

    // 3. Direct.
    None
}

/// Recompute the effective proxy and apply it to the process environment.
/// Call this whenever a relevant state change happens:
/// - app startup
/// - `download_proxy_auto` toggled
/// - system proxy enabled / disabled
/// - kernel started / stopped
pub fn update(state: &AppState) -> AppResult<()> {
    let proxy = resolve_effective_proxy(state);
    match proxy {
        Some(url) => {
            for var in PROXY_ENV_VARS {
                std::env::set_var(var, &url);
            }
        }
        None => {
            for var in PROXY_ENV_VARS {
                std::env::remove_var(var);
            }
        }
    }
    Ok(())
}
