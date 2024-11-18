use axum::{
    Router,
    routing::post,
};
use super::handlers::authenticate;

pub fn routes() -> Router {
    Router::new()
        .route("/auth", post(authenticate))
}
