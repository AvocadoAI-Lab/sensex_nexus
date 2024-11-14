use axum::Json;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};

pub async fn get_results(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "ciscat/{agent_id}/results", |url| url).await
}
