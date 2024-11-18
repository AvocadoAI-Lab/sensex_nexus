use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_rootcheck(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/{agent_id}", |url| url).await
}

pub async fn get_last_scan(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rootcheck/{agent_id}/last_scan", |url| url).await
}
