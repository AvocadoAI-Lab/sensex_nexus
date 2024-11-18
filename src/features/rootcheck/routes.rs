use axum::{
    routing::post,
    Router,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        .route("/rootcheck/:agent_id", post(get_rootcheck))
        .route("/rootcheck/:agent_id/last_scan", post(get_last_scan))
}
