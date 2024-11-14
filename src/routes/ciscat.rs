use axum::{
    routing::post,
    Router,
};
use crate::handlers::ciscat::*;

pub fn routes() -> Router {
    Router::new()
        .route("/ciscat/:agent_id/results", post(get_results))
}
