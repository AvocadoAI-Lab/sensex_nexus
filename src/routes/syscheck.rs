use axum::{
    routing::post,
    Router,
};
use crate::handlers::syscheck::*;

pub fn routes() -> Router {
    Router::new()
        .route("/syscheck/:agent_id", post(get_syscheck))
        .route("/syscheck/:agent_id/last_scan", post(get_last_scan))
}
