pub mod features;
pub mod shared;
pub mod client;

use axum::Router;
use axum::routing::get;

pub fn create_router() -> Router {
    Router::new()
        .merge(features::agents::routes())
        .merge(features::auth::routes())
        .merge(features::ciscat::routes())
        .merge(features::decoders::routes())
        .merge(features::groups::routes())
        .merge(features::lists::routes())
        .merge(features::manager::routes())
        .merge(features::mitre::routes())
        .merge(features::rootcheck::routes())
        .merge(features::rules::routes())
        .merge(features::sca::routes())
        .merge(features::security::routes())
        .merge(features::syscheck::routes())
        .merge(features::syscollector::routes())
        .merge(features::tasks::routes())
        .merge(features::wql::routes())
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
