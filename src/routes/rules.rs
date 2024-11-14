use axum::{
    Router,
    routing::post,
};
use crate::handlers::rules::*;

pub fn routes() -> Router {
    Router::new()
        // Rules information endpoints
        .route("/rules", post(get_rules))
        .route("/rules/groups", post(get_rules_groups))
        .route("/rules/files", post(get_rules_files))
}
