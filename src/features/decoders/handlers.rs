use axum::Json;
use crate::shared::common::{WazuhRequest, handle_wazuh_request};

// Decoders information endpoints
pub async fn get_decoders(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders", |url| url).await
}

pub async fn get_decoder_files(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/files", |url| url).await
}

pub async fn get_decoder_parents(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    handle_wazuh_request(payload, "decoders/parents", |url| url).await
}
