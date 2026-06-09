use serde_json::json;

use crate::kernel::zero::parsing::*;

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
fn parse_health_maps_engine_build_id() {
    let health = parse_health(&json!({
        "engine_build_id": "0.0.9",
        "started_at_unix_ms": 1713500000000_u64,
        "healthy": true
    }));

    assert!(health.healthy);
    assert_eq!(health.engine_version.as_deref(), Some("0.0.9"));
    assert_eq!(health.started_at_unix_ms, Some(1713500000000));
}

#[test]
fn parse_health_strips_v_prefix() {
    let health = parse_health(&json!({
        "engine_build_id": "v0.0.5",
        "healthy": true
    }));

    assert_eq!(health.engine_version.as_deref(), Some("0.0.5"));
}

#[test]
fn parse_health_defaults_healthy_when_missing() {
    let health = parse_health(&json!({}));

    assert!(health.healthy);
    assert!(health.engine_version.is_none());
    assert!(health.started_at_unix_ms.is_none());
}

#[test]
fn parse_capabilities_maps_api_id_field() {
    let caps = parse_capabilities(
        &json!({
            "api_id": "zero.api.v1",
            "schema_id": "zero.event.v1",
            "features": ["query", "config_snapshot"],
            "permissions": ["read"],
            "adapters": [{ "kind": "in_process", "enabled": true }],
            "sinks": []
        }),
        None,
    );

    assert!(caps.available);
    assert_eq!(caps.api_version.as_deref(), Some("zero.api.v1"));
    assert_eq!(caps.schema_version.as_deref(), Some("zero.event.v1"));
    assert_eq!(caps.features, vec!["query", "config_snapshot"]);
    assert_eq!(caps.adapters.len(), 1);
    assert_eq!(caps.adapters[0].kind, "in_process");
    assert!(caps.adapters[0].enabled);
}

#[test]
fn parse_capabilities_captures_error() {
    let caps = parse_capabilities(&json!({}), Some("connection refused".to_string()));

    assert!(!caps.available);
    assert_eq!(caps.error.as_deref(), Some("connection refused"));
}

#[test]
fn parse_connection_accepts_nested_target() {
    let conn = parse_connection(&json!({
        "flow_id": "42",
        "network": "tcp",
        "target": { "host": "example.com", "port": 443 },
        "traffic": { "bytes_up": 100, "bytes_down": 200 },
        "timing": { "started_at_unix_ms": 1713500000000_u64 }
    }))
    .unwrap();

    assert_eq!(conn.flow_id, "42");
    assert_eq!(conn.network, "tcp");
    assert_eq!(conn.destination, "example.com:443");
    assert_eq!(conn.bytes_up, 100);
    assert_eq!(conn.bytes_down, 200);
    assert_eq!(conn.started_at_unix_ms, Some(1713500000000));
}

#[test]
fn parse_connection_accepts_flat_destination() {
    let conn = parse_connection(&json!({
        "flow_id": "1",
        "host": "1.2.3.4",
        "port": 80,
        "inbound_tag": "socks5",
        "outbound_tag": "proxy"
    }))
    .unwrap();

    assert_eq!(conn.destination, "1.2.3.4:80");
    assert_eq!(conn.inbound_tag.as_deref(), Some("socks5"));
    assert_eq!(conn.outbound_tag.as_deref(), Some("proxy"));
}

#[test]
fn parse_connection_returns_none_without_flow_id() {
    assert!(parse_connection(&json!({ "host": "x" })).is_none());
}

#[test]
fn unwrap_core_envelope_strips_ok_result() {
    let value = unwrap_core_envelope(json!({
        "ok": true,
        "result": { "engine_build_id": "0.0.9" }
    }))
    .unwrap();

    assert_eq!(value["engine_build_id"], json!("0.0.9"));
}

#[test]
fn unwrap_core_envelope_rejects_ok_false() {
    let err = unwrap_core_envelope(json!({
        "ok": false,
        "error": { "code": "not_found", "message": "nope" }
    }))
    .unwrap_err();

    assert_eq!(err.code, "core_error");
}

#[test]
fn unwrap_core_envelope_passes_through_non_envelope() {
    let value = unwrap_core_envelope(json!({ "foo": "bar" })).unwrap();
    assert_eq!(value["foo"], json!("bar"));
}

// ── QueryResponse variant unwrapping tests ─────────────────────────

#[test]
fn unwrap_query_variant_strips_health_key() {
    let result = unwrap_query_variant(
        json!({
            "ok": true,
            "result": {
                "health": {
                    "engine_build_id": "0.0.9",
                    "started_at_unix_ms": 1713500000000_u64,
                    "healthy": true
                }
            }
        }),
        "health",
    )
    .unwrap();

    assert_eq!(result["engine_build_id"], json!("0.0.9"));
    assert_eq!(result["healthy"], json!(true));
}

#[test]
fn unwrap_query_variant_strips_capabilities_key() {
    let result = unwrap_query_variant(
        json!({
            "ok": true,
            "result": {
                "capabilities": {
                    "api_id": "zero.api.v1",
                    "features": ["query"],
                    "permissions": [],
                    "adapters": [],
                    "sinks": []
                }
            }
        }),
        "capabilities",
    )
    .unwrap();

    assert_eq!(result["api_id"], json!("zero.api.v1"));
    assert_eq!(result["features"], json!(["query"]));
}

#[test]
fn unwrap_query_variant_strips_active_flows_key() {
    let result = unwrap_query_variant(
        json!({
            "ok": true,
            "result": {
                "active_flows": [
                    { "flow_id": "1", "network": "tcp" },
                    { "flow_id": "2", "network": "udp" }
                ]
            }
        }),
        "active_flows",
    )
    .unwrap();

    assert!(result.is_array());
    assert_eq!(result.as_array().unwrap().len(), 2);
}

#[test]
fn unwrap_query_variant_strips_tun_status_key() {
    let result = unwrap_query_variant(
        json!({
            "ok": true,
            "result": {
                "tun_status": {
                    "running": true,
                    "name": "utun0",
                    "addr": "10.0.0.1/24",
                    "tag": "tun-in"
                }
            }
        }),
        "tun_status",
    )
    .unwrap();

    assert_eq!(result["running"], json!(true));
    assert_eq!(result["name"], json!("utun0"));
}

#[test]
fn unwrap_query_variant_falls_back_to_flat_shape() {
    let result = unwrap_query_variant(
        json!({
            "ok": true,
            "result": {
                "engine_build_id": "0.0.8",
                "healthy": true
            }
        }),
        "health",
    )
    .unwrap();

    // No "health" key inside result → returns the raw result
    assert_eq!(result["engine_build_id"], json!("0.0.8"));
}

#[test]
fn unwrap_query_variant_rejects_ok_false() {
    let err = unwrap_query_variant(
        json!({
            "ok": false,
            "error": { "code": "not_found", "message": "nope" }
        }),
        "health",
    )
    .unwrap_err();

    assert_eq!(err.code, "core_error");
}

#[test]
fn full_health_roundtrip_with_variant() {
    // Simulate the full IPC response → unwrap → parse pipeline
    let response = json!({
        "api_id": "zero.api.v1",
        "ok": true,
        "id": 1,
        "result": {
            "health": {
                "engine_build_id": "0.0.10",
                "started_at_unix_ms": 1713500000000_u64,
                "healthy": true
            }
        }
    });

    let inner = unwrap_query_variant(response, "health").unwrap();
    let health = parse_health(&inner);

    assert!(health.healthy);
    assert_eq!(health.engine_version.as_deref(), Some("0.0.10"));
    assert_eq!(health.started_at_unix_ms, Some(1713500000000));
}

#[test]
fn full_capabilities_roundtrip_with_variant() {
    let response = json!({
        "api_id": "zero.api.v1",
        "ok": true,
        "result": {
            "capabilities": {
                "api_id": "zero.api.v1",
                "schema_id": "zero.event.v1",
                "features": ["query", "config_snapshot", "runtime_snapshot"],
                "permissions": ["read"],
                "adapters": [{ "kind": "in_process", "enabled": true }],
                "sinks": []
            }
        }
    });

    let inner = unwrap_query_variant(response, "capabilities").unwrap();
    let caps = parse_capabilities(&inner, None);

    assert!(caps.available);
    assert_eq!(caps.features, vec!["query", "config_snapshot", "runtime_snapshot"]);
}

#[test]
fn full_active_flows_roundtrip_with_variant() {
    let response = json!({
        "api_id": "zero.api.v1",
        "ok": true,
        "result": {
            "active_flows": [
                {
                    "flow_id": "abc-123",
                    "network": "tcp",
                    "target": { "host": "example.com", "port": 443 },
                    "traffic": { "bytes_up": 500, "bytes_down": 1500 }
                }
            ]
        }
    });

    let inner = unwrap_query_variant(response, "active_flows").unwrap();
    let list = parse_connection_list(&inner, 100);

    assert_eq!(list.items.len(), 1);
    assert_eq!(list.items[0].flow_id, "abc-123");
    assert_eq!(list.items[0].destination, "example.com:443");
}

#[test]
fn string_at_finds_first_matching_key() {
    let v = json!({ "b": "found" });
    assert_eq!(string_at(&v, &["a", "b", "c"]), Some("found".to_string()));
}

#[test]
fn u64_at_handles_string_numbers() {
    let v = json!({ "count": "42" });
    assert_eq!(u64_at(&v, &["count"]), Some(42));
}

#[test]
fn bool_at_handles_string_booleans() {
    let v = json!({ "flag": "yes" });
    assert_eq!(bool_at(&v, &["flag"]), Some(true));
}

#[test]
fn normalize_version_strips_v_prefix() {
    assert_eq!(normalize_version(Some("v0.0.5".to_string())), Some("0.0.5".to_string()));
    assert_eq!(normalize_version(Some("0.0.5".to_string())), Some("0.0.5".to_string()));
    assert_eq!(normalize_version(None), None);
}

// ── plan_apply result parsing tests ──────────────────────────────────

#[test]
fn plan_apply_parses_full_impact_analysis() {
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "hot_reload": [
            { "section": "outbounds", "tags": ["proxy-us", "proxy-jp"], "detail": "2 outbound proxies updated" },
            { "section": "rules", "tags": [], "detail": "5 routing rules changed" }
        ],
        "requires_restart": [
            { "section": "listeners", "tags": ["mixed-in"], "detail": "listening port changed from 1080 to 1081" }
        ],
        "warnings": ["Active connections may be interrupted"],
        "errors": []
    }));

    assert!(result.valid);
    assert_eq!(result.hot_reload.len(), 2);
    assert_eq!(result.hot_reload[0].section, "outbounds");
    assert_eq!(result.hot_reload[0].tags, vec!["proxy-us", "proxy-jp"]);
    assert_eq!(result.hot_reload[0].detail, "2 outbound proxies updated");
    assert_eq!(result.hot_reload[1].section, "rules");
    assert!(result.hot_reload[1].tags.is_empty());
    assert_eq!(result.requires_restart.len(), 1);
    assert_eq!(result.requires_restart[0].section, "listeners");
    assert_eq!(result.requires_restart[0].tags, vec!["mixed-in"]);
    assert_eq!(result.warnings, vec!["Active connections may be interrupted"]);
    assert!(result.errors.is_empty());
}

#[test]
fn plan_apply_parses_hot_reload_only() {
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "hot_reload": [
            { "section": "outbounds", "tags": ["proxy-sg"], "detail": "1 outbound updated" }
        ],
        "requires_restart": [],
        "warnings": [],
        "errors": []
    }));

    assert!(result.valid);
    assert_eq!(result.hot_reload.len(), 1);
    assert!(result.requires_restart.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn plan_apply_parses_restart_only() {
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "hot_reload": [],
        "requires_restart": [
            { "section": "tun", "tags": [], "detail": "TUN address changed" },
            { "section": "dns", "tags": [], "detail": "DNS server changed" }
        ],
        "warnings": ["Kernel must be restarted for TUN changes"],
        "errors": []
    }));

    assert!(result.valid);
    assert!(result.hot_reload.is_empty());
    assert_eq!(result.requires_restart.len(), 2);
    assert_eq!(result.requires_restart[0].section, "tun");
    assert_eq!(result.requires_restart[1].section, "dns");
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn plan_apply_parses_invalid_config() {
    let result = parse_plan_apply_result(&json!({
        "valid": false,
        "hot_reload": [],
        "requires_restart": [],
        "warnings": [],
        "errors": ["unknown field `foobar` at line 42", "missing required field `outbounds`"]
    }));

    assert!(!result.valid);
    assert!(result.hot_reload.is_empty());
    assert!(result.requires_restart.is_empty());
    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0], "unknown field `foobar` at line 42");
}

#[test]
fn plan_apply_unwraps_result_envelope() {
    // Kernel may return { ok: true, result: { valid, hot_reload, ... } }
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "result": {
            "valid": true,
            "hot_reload": [
                { "section": "config", "tags": [], "detail": "all changes" }
            ],
            "requires_restart": [],
            "warnings": [],
            "errors": []
        }
    }));

    // When outer `result` key is present, the parser should unwrap into it
    assert!(result.valid);
    assert_eq!(result.hot_reload.len(), 1);
    assert_eq!(result.hot_reload[0].section, "config");
    assert!(result.requires_restart.is_empty());
}

#[test]
fn plan_apply_defaults_to_empty_on_minimal_input() {
    let result = parse_plan_apply_result(&json!({
        "valid": true
    }));

    assert!(result.valid);
    assert!(result.hot_reload.is_empty());
    assert!(result.requires_restart.is_empty());
    assert!(result.warnings.is_empty());
    assert!(result.errors.is_empty());
}

#[test]
fn plan_apply_defaults_valid_to_true_when_missing() {
    let result = parse_plan_apply_result(&json!({}));

    // Tolerant: missing `valid` defaults to true
    assert!(result.valid);
    assert!(result.hot_reload.is_empty());
    assert!(result.requires_restart.is_empty());
}

#[test]
fn plan_apply_tolerates_missing_detail_in_items() {
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "hot_reload": [
            { "section": "outbounds", "tags": ["proxy-us"] }
        ],
        "requires_restart": [
            { "section": "listeners" }
        ],
        "warnings": [],
        "errors": []
    }));

    assert_eq!(result.hot_reload[0].detail, "");
    assert!(result.hot_reload[0].tags.is_empty() == false);
    assert_eq!(result.requires_restart[0].detail, "");
    assert!(result.requires_restart[0].tags.is_empty());
}

#[test]
fn plan_accepts_alternative_item_key_names() {
    // Parser accepts "key"/"name" as aliases for "section",
    // "description"/"message" as aliases for "detail",
    // "affected" as alias for "tags"
    let result = parse_plan_apply_result(&json!({
        "valid": true,
        "hot_reload": [
            { "name": "routing", "affected": ["rule-a", "rule-b"], "description": "2 rules changed" }
        ],
        "requires_restart": [
            { "key": "dns", "message": "DNS upstream changed" }
        ],
        "warnings": ["Some warning"],
        "errors": []
    }));

    assert_eq!(result.hot_reload[0].section, "routing");
    assert_eq!(result.hot_reload[0].tags, vec!["rule-a", "rule-b"]);
    assert_eq!(result.hot_reload[0].detail, "2 rules changed");
    assert_eq!(result.requires_restart[0].section, "dns");
    assert_eq!(result.requires_restart[0].detail, "DNS upstream changed");
}
