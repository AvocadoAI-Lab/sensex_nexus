use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// CDB lists information endpoints
pub async fn get_lists(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists", |url| url).await
}

pub async fn get_lists_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "lists/files", |url| url).await
}
