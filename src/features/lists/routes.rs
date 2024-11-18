use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // CDB lists information endpoints
        .route("/lists", post(get_lists))
        .route("/lists/files", post(get_lists_files))
}
