use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Base agents endpoint
pub async fn get_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents", |url| url).await
}

// Agent configuration and stats
pub async fn get_agent_config_by_id(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/{agent_id}/config/{component}/{configuration}", |url| url).await
}

pub async fn get_agent_group_sync_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/{agent_id}/group/is_sync", |url| url).await
}

pub async fn get_daemon_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/{agent_id}/daemons/stats", |url| url).await
}

pub async fn get_agent_stats_component(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/{agent_id}/stats/{component}", |url| url).await
}

// Group related endpoints
pub async fn get_agents_without_group(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/no_group", |url| url).await
}

// Status and summary endpoints
pub async fn get_outdated_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/outdated", |url| url).await
}

pub async fn get_distinct_agents_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/stats/distinct", |url| url).await
}

pub async fn get_agents_os_summary(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/summary/os", |url| url).await
}

pub async fn get_agents_status_summary(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "agents/summary/status", |url| url).await
}
