use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_sca(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "sca/{agent_id}", |url| url).await
}

pub async fn get_checks(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "sca/{agent_id}/checks/{policy_id}", |url| url).await
}
