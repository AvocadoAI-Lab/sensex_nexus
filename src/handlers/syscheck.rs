use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_syscheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck/{agent_id}", |url| url).await
}

pub async fn get_last_scan(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "syscheck/{agent_id}/last_scan", |url| url).await
}
