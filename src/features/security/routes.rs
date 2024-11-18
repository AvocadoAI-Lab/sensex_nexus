use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Security information endpoints
        .route("/security/actions", post(get_security_actions))
        .route("/security/resources", post(get_security_resources))
        .route("/security/config", post(get_security_config))
}
