use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

// Rules information endpoints
pub async fn get_rules(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules", |url| url).await
}

pub async fn get_rules_groups(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/groups", |url| url).await
}

pub async fn get_rules_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "rules/files", |url| url).await
}
