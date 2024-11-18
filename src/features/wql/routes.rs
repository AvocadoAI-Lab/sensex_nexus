use axum::{
    Router,
    routing::post,
    extract::Path,
};
use super::handlers::handle_wql_query;

pub fn routes() -> Router {
    Router::new()
        .route("/wql/:group", post(|Path(group): Path<String>| handle_wql_query(group)))
}
