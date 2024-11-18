use axum::{
    routing::post,
    Router,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        .route("/ciscat/:agent_id/results", post(get_results))
}
