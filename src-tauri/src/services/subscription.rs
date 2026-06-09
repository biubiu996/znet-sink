use std::time::Duration;
use tauri::State;

use base64::{Engine as _, engine::general_purpose};
use serde_json::{Map, Value, json};

use crate::errors::{AppError, AppResult};
use crate::models::proxy_config::ProxyConfigProfile;
use crate::models::subscription::{SubscriptionProfile, SubscriptionUpsert};
use crate::services::common::{
    generated_store_id, lock, normalize_optional, normalize_required, now_unix_ms,
};
use crate::services::domain_store;
use crate::services::proxy_config;
use crate::state::app_state::AppState;

const SUBSCRIPTION_FETCH_TIMEOUT_SECONDS: u64 = 30;

pub fn list(state: State<'_, AppState>) -> AppResult<Vec<SubscriptionProfile>> {
    Ok(lock(state.subscriptions(), "subscription")?.clone())
}

pub fn get(state: State<'_, AppState>, id: String) -> AppResult<SubscriptionProfile> {
    let id = normalize_required(id, "id")?;
    lock(state.subscriptions(), "subscription")?
        .iter()
        .find(|profile| profile.id == id)
        .cloned()
        .ok_or_else(|| AppError::not_found("subscription", id))
}

pub fn upsert(
    state: State<'_, AppState>,
    input: SubscriptionUpsert,
) -> AppResult<SubscriptionProfile> {
    let name = normalize_required(input.name, "name")?;
    let url = normalize_required(input.url, "url")?;
    validate_http_url(&url)?;

    let id = normalize_optional(input.id).unwrap_or_else(|| generated_store_id("subscription"));
    let profile = SubscriptionProfile {
        id: id.clone(),
        name,
        url,
        enabled: input.enabled.unwrap_or(true),
        kernel: normalize_optional(input.kernel).unwrap_or_else(|| "zero".to_string()),
        format: normalize_optional(input.format).unwrap_or_else(|| "auto".to_string()),
        target_proxy_config_id: normalize_optional(input.target_proxy_config_id),
        updated_at_unix_ms: now_unix_ms(),
        last_sync_at_unix_ms: None,
        last_error: None,
    };

    let mut subscriptions = lock(state.subscriptions(), "subscription")?;
    match subscriptions.iter_mut().find(|item| item.id == id) {
        Some(existing) => {
            let last_sync_at_unix_ms = existing.last_sync_at_unix_ms;
            *existing = SubscriptionProfile {
                last_sync_at_unix_ms,
                ..profile.clone()
            };
        }
        None => subscriptions.push(profile.clone()),
    }
    domain_store::save_subscriptions(&subscriptions)?;

    Ok(profile)
}

pub async fn sync(state: State<'_, AppState>, id: String) -> AppResult<SubscriptionProfile> {
    let id = normalize_required(id, "id")?;
    let subscription = {
        let subscriptions = lock(state.subscriptions(), "subscription")?;
        subscriptions
            .iter()
            .find(|profile| profile.id == id)
            .cloned()
            .ok_or_else(|| AppError::not_found("subscription", id.clone()))?
    };

    if !subscription.enabled {
        let error = AppError::invalid_argument("subscription is disabled");
        update_sync_error(state.inner(), &id, &error.message)?;
        return Err(error);
    }

    let result = sync_subscription(state.inner(), subscription).await;
    if let Err(error) = &result {
        update_sync_error(state.inner(), &id, &error.message)?;
    }

    result
}

pub fn remove(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let id = normalize_required(id, "id")?;
    let mut subscriptions = lock(state.subscriptions(), "subscription")?;
    let before = subscriptions.len();
    subscriptions.retain(|profile| profile.id != id);

    if subscriptions.len() == before {
        return Err(AppError::not_found("subscription", id));
    }
    domain_store::save_subscriptions(&subscriptions)?;

    Ok(())
}

fn validate_http_url(url: &str) -> AppResult<()> {
    if url.starts_with("https://") || url.starts_with("http://") {
        return Ok(());
    }

    Err(AppError::invalid_argument(
        "subscription url must start with http:// or https://",
    ))
}

async fn sync_subscription(
    state: &AppState,
    subscription: SubscriptionProfile,
) -> AppResult<SubscriptionProfile> {
    let content = fetch_subscription_content(subscription.url.clone()).await?;
    let parsed = parse_subscription_content(&content, &subscription.format)?;
    let now = now_unix_ms();
    let target_proxy_config_id = subscription
        .target_proxy_config_id
        .clone()
        .unwrap_or_else(|| generated_store_id("proxy-config"));

    upsert_synced_proxy_config(state, &subscription, &target_proxy_config_id, parsed, now)?;
    update_sync_success(state, &subscription.id, target_proxy_config_id, now)
}

async fn fetch_subscription_content(url: String) -> AppResult<String> {
    tauri::async_runtime::spawn_blocking(move || fetch_subscription_content_blocking(&url))
        .await
        .map_err(|error| AppError::internal(format!("subscription worker failed: {error}")))?
}

fn fetch_subscription_content_blocking(url: &str) -> AppResult<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(SUBSCRIPTION_FETCH_TIMEOUT_SECONDS))
        .user_agent("ZNet Sink")
        .build()
        .map_err(|error| AppError::internal(format!("failed to build HTTP client: {error}")))?;

    let response = client.get(url).send().map_err(|error| AppError {
        code: "upstream_error",
        message: format!("failed to fetch subscription: {error}"),
        details: Some(serde_json::json!({ "url": url })),
    })?;

    let status = response.status();
    if !status.is_success() {
        return Err(AppError {
            code: "upstream_error",
            message: format!("subscription server returned HTTP {status}"),
            details: Some(serde_json::json!({ "url": url, "status": status.as_u16() })),
        });
    }

    response.text().map_err(|error| AppError {
        code: "upstream_error",
        message: format!("failed to read subscription response: {error}"),
        details: Some(serde_json::json!({ "url": url })),
    })
}

#[derive(Clone, Debug)]
pub struct ParsedSubscriptionConfig {
    pub content: serde_json::Value,
    pub format: String,
}

pub fn parse_subscription_content(
    content: &str,
    format: &str,
) -> AppResult<ParsedSubscriptionConfig> {
    let content = content.trim();
    if content.is_empty() {
        return Err(AppError::invalid_argument(
            "subscription response must not be empty",
        ));
    }

    let format = format.trim().to_ascii_lowercase();
    match format.as_str() {
        "" | "auto" => parse_auto_subscription_content(content),
        "zero" | "zero-base64-json" | "base64-json" => parse_base64_json_subscription_content(content),
        "clash" | "clash-yaml" | "yaml" => parse_clash_yaml_subscription_content(content),
        _ => Err(AppError::invalid_argument(format!(
            "unsupported subscription format: {format}"
        ))),
    }
}

fn parse_auto_subscription_content(content: &str) -> AppResult<ParsedSubscriptionConfig> {
    parse_base64_json_subscription_content(content)
        .or_else(|_| parse_clash_yaml_subscription_content(content))
}

fn parse_base64_json_subscription_content(content: &str) -> AppResult<ParsedSubscriptionConfig> {
    let decoded = decode_base64(content)?;
    let decoded = String::from_utf8(decoded).map_err(|error| AppError {
        code: "invalid_argument",
        message: format!("subscription decoded content is not valid UTF-8: {error}"),
        details: None,
    })?;

    let content: serde_json::Value = serde_json::from_str(&decoded).map_err(|error| AppError {
        code: "invalid_argument",
        message: format!("subscription decoded JSON is invalid: {error}"),
        details: None,
    })?;
    if !content.is_object() {
        return Err(AppError::invalid_argument(
            "subscription decoded JSON must be an object",
        ));
    }

    Ok(ParsedSubscriptionConfig {
        content,
        format: "zero-base64-json".to_string(),
    })
}

fn parse_clash_yaml_subscription_content(content: &str) -> AppResult<ParsedSubscriptionConfig> {
    let clash: Value = serde_yaml::from_str(content).map_err(|error| AppError {
        code: "invalid_argument",
        message: format!("subscription Clash YAML is invalid: {error}"),
        details: None,
    })?;

    let content = convert_clash_to_zero(&clash)?;
    Ok(ParsedSubscriptionConfig {
        content,
        format: "clash-yaml-converted".to_string(),
    })
}

fn convert_clash_to_zero(clash: &Value) -> AppResult<Value> {
    let root = clash.as_object().ok_or_else(|| {
        AppError::invalid_argument("subscription Clash YAML root must be an object")
    })?;

    let proxies = root
        .get("proxies")
        .and_then(Value::as_array)
        .ok_or_else(|| AppError::invalid_argument("subscription Clash YAML must contain proxies"))?;

    let mut outbounds = Vec::new();
    outbounds.push(json!({ "tag": "direct", "type": "direct" }));
    outbounds.push(json!({ "tag": "block", "type": "block" }));
    outbounds.extend(proxies.iter().filter_map(convert_clash_proxy));

    let mut known_tags = outbounds
        .iter()
        .filter_map(|outbound| outbound.get("tag").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect::<std::collections::BTreeSet<_>>();

    let outbound_groups = root
        .get("proxy-groups")
        .and_then(Value::as_array)
        .map(|groups| {
            groups
                .iter()
                .filter_map(|group| convert_clash_proxy_group(group, &known_tags))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    for group in &outbound_groups {
        if let Some(tag) = group.get("tag").and_then(Value::as_str) {
            known_tags.insert(tag.to_string());
        }
    }

    let rules = root
        .get("rules")
        .and_then(Value::as_array)
        .map(|rules| {
            rules
                .iter()
                .filter_map(|rule| convert_clash_rule(rule, &known_tags))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let final_outbound = root
        .get("rules")
        .and_then(Value::as_array)
        .and_then(|rules| {
            rules
                .iter()
                .rev()
                .find_map(|rule| clash_match_outbound(rule, &known_tags))
        })
        .unwrap_or_else(|| {
            outbound_groups
                .first()
                .and_then(|group| group.get("tag").and_then(Value::as_str))
                .unwrap_or("direct")
                .to_string()
        });

    Ok(json!({
        "outbounds": outbounds,
        "outbound_groups": outbound_groups,
        "route": {
            "rules": rules,
            "final": { "type": "route", "outbound": final_outbound }
        }
    }))
}

fn convert_clash_proxy(proxy: &Value) -> Option<Value> {
    let source = proxy.as_object()?;
    let tag = string_field(source, "name")?;
    let proxy_type = string_field(source, "type")?.to_ascii_lowercase();

    let mapped_type = match proxy_type.as_str() {
        "ss" | "shadowsocks" => "shadowsocks",
        "ssr" => "shadowsocksr",
        "vmess" => "vmess",
        "vless" => "vless",
        "trojan" => "trojan",
        "socks5" | "socks" => "socks",
        "http" | "https" => "http",
        "hysteria" | "hysteria2" | "tuic" | "wireguard" => proxy_type.as_str(),
        _ => proxy_type.as_str(),
    };

    let mut outbound = Map::new();
    outbound.insert("tag".to_string(), Value::String(tag));
    outbound.insert("type".to_string(), Value::String(mapped_type.to_string()));

    for (key, value) in source {
        if key == "name" || key == "type" {
            continue;
        }
        outbound.insert(normalize_clash_key(key), value.clone());
    }

    Some(Value::Object(outbound))
}

fn convert_clash_proxy_group(
    group: &Value,
    known_tags: &std::collections::BTreeSet<String>,
) -> Option<Value> {
    let source = group.as_object()?;
    let tag = string_field(source, "name")?;
    let group_type = string_field(source, "type")
        .unwrap_or_else(|| "select".to_string())
        .to_ascii_lowercase();
    let outbounds = source
        .get("proxies")
        .and_then(Value::as_array)?
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|tag| normalize_outbound_ref(tag, known_tags))
        .map(Value::String)
        .collect::<Vec<_>>();

    if outbounds.is_empty() {
        return None;
    }

    let mapped_type = match group_type.as_str() {
        "url-test" => "urltest",
        "fallback" => "fallback",
        "load-balance" => "loadbalance",
        _ => "selector",
    };

    let mut converted = Map::new();
    converted.insert("tag".to_string(), Value::String(tag));
    converted.insert("type".to_string(), Value::String(mapped_type.to_string()));
    converted.insert("outbounds".to_string(), Value::Array(outbounds));

    if let Some(url) = source.get("url") {
        converted.insert("url".to_string(), url.clone());
    }
    if let Some(interval) = source.get("interval") {
        converted.insert("interval".to_string(), interval.clone());
    }

    Some(Value::Object(converted))
}

fn convert_clash_rule(
    rule: &Value,
    known_tags: &std::collections::BTreeSet<String>,
) -> Option<Value> {
    let raw = rule.as_str()?.trim();
    let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
    if parts.len() < 2 {
        return None;
    }

    let rule_type = parts[0].to_ascii_uppercase();
    if rule_type == "MATCH" {
        return None;
    }

    let outbound = normalize_outbound_ref(parts.last()?, known_tags)?;

    let value = parts.get(1)?.to_string();
    let condition = match rule_type.as_str() {
        "DOMAIN" => json!({ "type": "domain", "values": [value] }),
        "DOMAIN-SUFFIX" => json!({ "type": "domain_suffix", "values": [value] }),
        "DOMAIN-KEYWORD" => json!({ "type": "domain_keyword", "values": [value] }),
        "IP-CIDR" | "IP-CIDR6" => json!({ "type": "ip_cidr", "values": [value] }),
        "SRC-IP-CIDR" => json!({ "type": "source_ip_cidr", "values": [value] }),
        "GEOIP" => json!({ "type": "geoip", "values": [value] }),
        "GEOSITE" => json!({ "type": "geosite", "values": [value] }),
        "RULE-SET" => json!({ "type": "rule_set", "tag": value }),
        _ => return None,
    };

    Some(json!({
        "condition": condition,
        "action": { "type": "route", "outbound": outbound }
    }))
}

fn clash_match_outbound(
    rule: &Value,
    known_tags: &std::collections::BTreeSet<String>,
) -> Option<String> {
    let raw = rule.as_str()?.trim();
    let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
    if parts.first().is_some_and(|part| part.eq_ignore_ascii_case("MATCH")) {
        return normalize_outbound_ref(parts.last()?, known_tags);
    }
    None
}

fn normalize_outbound_ref(
    tag: &str,
    known_tags: &std::collections::BTreeSet<String>,
) -> Option<String> {
    if known_tags.contains(tag) {
        return Some(tag.to_string());
    }

    match tag.to_ascii_uppercase().as_str() {
        "DIRECT" => Some("direct".to_string()),
        "REJECT" => Some("block".to_string()),
        _ => None,
    }
}

fn string_field(map: &Map<String, Value>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn normalize_clash_key(key: &str) -> String {
    key.replace('-', "_")
}


fn decode_base64(content: &str) -> AppResult<Vec<u8>> {
    let compact = content.split_whitespace().collect::<String>();
    if compact.is_empty() {
        return Err(AppError::invalid_argument(
            "subscription response must not be empty",
        ));
    }

    let padded = pad_base64(&compact);
    general_purpose::STANDARD
        .decode(&padded)
        .or_else(|_| general_purpose::URL_SAFE.decode(&padded))
        .map_err(|error| AppError {
            code: "invalid_argument",
            message: format!("subscription response must be base64 encoded JSON: {error}"),
            details: None,
        })
}

fn pad_base64(content: &str) -> String {
    let mut padded = content.to_string();
    let remainder = padded.len() % 4;
    if remainder != 0 {
        padded.extend(std::iter::repeat_n('=', 4 - remainder));
    }
    padded
}

fn upsert_synced_proxy_config(
    state: &AppState,
    subscription: &SubscriptionProfile,
    target_proxy_config_id: &str,
    parsed: ParsedSubscriptionConfig,
    updated_at_unix_ms: u64,
) -> AppResult<ProxyConfigProfile> {
    let capabilities = proxy_config::analyze_capabilities(Some(&parsed.content));
    let mut profiles = lock(state.proxy_configs(), "proxy_config")?;
    let existing_active = profiles
        .iter()
        .find(|profile| profile.id == target_proxy_config_id)
        .is_some_and(|profile| profile.active);
    let profile = ProxyConfigProfile {
        id: target_proxy_config_id.to_string(),
        name: subscription.name.clone(),
        kernel: subscription.kernel.clone(),
        format: parsed.format,
        path: Some(subscription.url.clone()),
        content: Some(parsed.content),
        active: existing_active,
        updated_at_unix_ms,
        capabilities,
    };

    match profiles
        .iter_mut()
        .find(|profile| profile.id == target_proxy_config_id)
    {
        Some(existing) => *existing = profile.clone(),
        None => profiles.push(profile.clone()),
    }
    domain_store::save_proxy_configs(&profiles)?;
    if profile.active {
        proxy_config::sync_local_proxy_from_profile(state, &profile)?;
    }

    Ok(profile)
}

fn update_sync_success(
    state: &AppState,
    id: &str,
    target_proxy_config_id: String,
    synced_at_unix_ms: u64,
) -> AppResult<SubscriptionProfile> {
    let mut subscriptions = lock(state.subscriptions(), "subscription")?;
    let subscription = subscriptions
        .iter_mut()
        .find(|profile| profile.id == id)
        .ok_or_else(|| AppError::not_found("subscription", id.to_string()))?;

    subscription.target_proxy_config_id = Some(target_proxy_config_id);
    subscription.last_sync_at_unix_ms = Some(synced_at_unix_ms);
    subscription.last_error = None;
    subscription.updated_at_unix_ms = synced_at_unix_ms;
    let updated = subscription.clone();
    domain_store::save_subscriptions(&subscriptions)?;

    Ok(updated)
}

fn update_sync_error(state: &AppState, id: &str, message: &str) -> AppResult<()> {
    let mut subscriptions = lock(state.subscriptions(), "subscription")?;
    if let Some(subscription) = subscriptions.iter_mut().find(|profile| profile.id == id) {
        subscription.last_error = Some(message.to_string());
        subscription.updated_at_unix_ms = now_unix_ms();
        domain_store::save_subscriptions(&subscriptions)?;
    }

    Ok(())
}
