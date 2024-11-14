use axum::{
    Router,
    routing::post,
};
use crate::handlers::decoders::*;

pub fn routes() -> Router {
    Router::new()
        // Decoders information endpoints
        .route("/decoders", post(get_decoders))
        .route("/decoders/files", post(get_decoder_files))
        .route("/decoders/parents", post(get_decoder_parents))
}
