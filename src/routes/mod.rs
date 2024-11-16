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
mod syscheck;
mod sca;
mod rootcheck;
mod ciscat;
mod wql;        // New

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
        .merge(syscheck::routes())
        .merge(sca::routes())
        .merge(rootcheck::routes())
        .merge(ciscat::routes())
        .merge(wql::routes())    // New
        .route("/health", get(health_check))
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
