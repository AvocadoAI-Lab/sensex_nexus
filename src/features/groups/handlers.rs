use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

// Groups information endpoints
pub async fn get_groups(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups", |url| url).await
}

pub async fn get_group_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/files", |url| url).await
}

pub async fn get_group_agents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/agents", |url| url).await
}

pub async fn get_group_configuration(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "groups/{group_id}/configuration", |url| url).await
}
