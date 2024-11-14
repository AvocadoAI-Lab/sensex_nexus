use axum::{
    Router,
    routing::post,
};
use crate::handlers::tasks::*;

pub fn routes() -> Router {
    Router::new()
        // Tasks information endpoint
        .route("/tasks/status", post(get_tasks_status))
}
