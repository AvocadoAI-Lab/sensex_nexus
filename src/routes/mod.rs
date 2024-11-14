use axum::Router;
use axum::routing::get;

mod agents;
mod auth;
mod decoders;
mod groups;
mod lists;
mod manager;
mod mitre;
mod rules;
mod security;
mod syscollector;
mod tasks;
mod syscheck;    // New
mod sca;         // New
mod rootcheck;   // New
mod ciscat;      // New

pub fn create_router() -> Router {
    Router::new()
        .merge(agents::routes())
        .merge(auth::routes())
        .merge(decoders::routes())
        .merge(groups::routes())
        .merge(lists::routes())
        .merge(manager::routes())
        .merge(mitre::routes())
        .merge(rules::routes())
        .merge(security::routes())
        .merge(syscollector::routes())
        .merge(tasks::routes())
        .merge(syscheck::routes())    // New
        .merge(sca::routes())         // New
        .merge(rootcheck::routes())   // New
        .merge(ciscat::routes())      // New
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
