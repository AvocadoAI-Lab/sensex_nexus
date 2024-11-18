use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

// Security information endpoints
pub async fn get_security_actions(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/actions", |url| url).await
}

pub async fn get_security_resources(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/resources", |url| url).await
}

pub async fn get_security_config(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "security/config", |url| url).await
}
