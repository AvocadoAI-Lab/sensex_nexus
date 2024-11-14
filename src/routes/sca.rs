use axum::{
    routing::post,
    Router,
};
use crate::handlers::sca::*;

pub fn routes() -> Router {
    Router::new()
        .route("/sca/:agent_id", post(get_sca))
        .route("/sca/:agent_id/checks/:policy_id", post(get_checks))
}
