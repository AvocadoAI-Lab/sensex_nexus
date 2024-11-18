use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

// Hardware information
pub async fn get_syscollector_hardware(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/hardware", |url| url).await
}

// Hotfixes information
pub async fn get_syscollector_hotfixes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/hotfixes", |url| url).await
}

// Network information
pub async fn get_syscollector_netaddr(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/netaddr", |url| url).await
}

pub async fn get_syscollector_netiface(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/netiface", |url| url).await
}

pub async fn get_syscollector_netproto(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/netproto", |url| url).await
}

// Operating system information
pub async fn get_syscollector_os(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/os", |url| url).await
}

// Package information
pub async fn get_syscollector_packages(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/packages", |url| url).await
}

// Port information
pub async fn get_syscollector_ports(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/ports", |url| url).await
}

// Process information
pub async fn get_syscollector_processes(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscollector/{agent_id}/processes", |url| url).await
}
