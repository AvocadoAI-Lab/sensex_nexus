use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

// Get status of tasks
pub async fn get_tasks_status(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "tasks/status", |url| url).await
}
