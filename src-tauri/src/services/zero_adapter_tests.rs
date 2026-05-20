use serde_json::json;

use crate::models::gui_core::GuiTrafficStats;
use crate::services::zero_adapter::{
    build_traffic_snapshot, bytes_delta_per_second, calculate_rates, parse_stats,
};
use crate::state::app_state::AppState;

#[test]
fn parse_stats_accepts_current_zero_fields() {
    let stats = parse_stats(&json!({
        "active_sessions": 2,
        "total_started": 10,
        "completed_sessions": 7,
        "failed_sessions": 1,
        "blocked_sessions": 1,
        "direct_sessions": 3,
        "chained_sessions": 4,
        "bytes_up": 1200,
        "bytes_down": 3400
    }));

    assert_eq!(stats.active_sessions, 2);
    assert_eq!(stats.total_started, 10);
    assert_eq!(stats.bytes_up, 1200);
    assert_eq!(stats.bytes_down, 3400);
}

#[test]
fn traffic_rates_use_byte_delta_over_interval() {
    let previous = GuiTrafficStats {
        bytes_up: 1_000,
        bytes_down: 10_000,
        ..GuiTrafficStats::default()
    };
    let current = GuiTrafficStats {
        bytes_up: 3_000,
        bytes_down: 15_000,
        ..GuiTrafficStats::default()
    };

    let rates = calculate_rates(&previous, &current, 2_000);

    assert_eq!(rates.upload_bps, 1_000);
    assert_eq!(rates.download_bps, 2_500);
}

#[test]
fn traffic_rates_treat_counter_reset_as_zero_rate() {
    assert_eq!(bytes_delta_per_second(10_000, 5_000, 1_000), 0);
}

#[test]
fn first_traffic_snapshot_is_unstable_baseline() {
    let state = AppState::default();
    let snapshot = build_traffic_snapshot(
        &state,
        GuiTrafficStats {
            bytes_up: 100,
            bytes_down: 200,
            ..GuiTrafficStats::default()
        },
        1_000,
    )
    .unwrap();

    assert!(!snapshot.stable);
    assert_eq!(snapshot.rates.upload_bps, 0);
    assert!(snapshot.interval_ms.is_none());
}

#[test]
fn second_traffic_snapshot_reports_rates() {
    let state = AppState::default();
    build_traffic_snapshot(
        &state,
        GuiTrafficStats {
            bytes_up: 100,
            bytes_down: 200,
            ..GuiTrafficStats::default()
        },
        1_000,
    )
    .unwrap();

    let snapshot = build_traffic_snapshot(
        &state,
        GuiTrafficStats {
            bytes_up: 1_100,
            bytes_down: 2_200,
            ..GuiTrafficStats::default()
        },
        2_000,
    )
    .unwrap();

    assert!(snapshot.stable);
    assert_eq!(snapshot.interval_ms, Some(1_000));
    assert_eq!(snapshot.rates.upload_bps, 1_000);
    assert_eq!(snapshot.rates.download_bps, 2_000);
}
