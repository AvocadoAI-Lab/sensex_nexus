use axum::{
    Router,
    routing::post,
};

use crate::handlers::wql::handle_wql_query;

pub fn routes() -> Router {
    Router::new()
        .route("/wql/:group", post(handle_wql_query))
}
