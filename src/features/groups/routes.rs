use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Groups information endpoints
        .route("/groups", post(get_groups))
        .route("/groups/:group_id/files", post(get_group_files))
        .route("/groups/:group_id/agents", post(get_group_agents))
        .route("/groups/:group_id/configuration", post(get_group_configuration))
}
