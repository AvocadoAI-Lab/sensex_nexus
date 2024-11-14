use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// API info
pub async fn get_api_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "", |url| url).await
}

// Basic manager information
pub async fn get_manager_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/status", |url| url).await
}

pub async fn get_manager_info(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/info", |url| url).await
}

// Configuration
pub async fn get_manager_configuration(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/configuration", |url| url).await
}

// Statistics
pub async fn get_manager_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats", |url| url).await
}

pub async fn get_manager_hourly_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/hourly", |url| url).await
}

pub async fn get_manager_weekly_stats(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/stats/weekly", |url| url).await
}

// Logs
pub async fn get_manager_logs(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/logs", |url| url).await
}

pub async fn get_manager_logs_summary(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "manager/logs/summary", |url| url).await
}
