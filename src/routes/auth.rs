use axum::{
    Router,
    routing::post,
};
use crate::handlers::auth::authenticate;

pub fn routes() -> Router {
    Router::new()
        .route("/auth", post(authenticate))
}
