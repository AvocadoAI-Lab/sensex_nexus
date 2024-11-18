use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Tasks information endpoint
        .route("/tasks/status", post(get_tasks_status))
}
