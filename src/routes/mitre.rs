use axum::{
    Router,
    routing::post,
};
use crate::handlers::mitre::*;

pub fn routes() -> Router {
    Router::new()
        // MITRE information endpoints
        .route("/mitre/groups", post(get_mitre_groups))
        .route("/mitre/metadata", post(get_mitre_metadata))
        .route("/mitre/mitigations", post(get_mitre_mitigations))
        .route("/mitre/references", post(get_mitre_references))
        .route("/mitre/software", post(get_mitre_software))
        .route("/mitre/tactics", post(get_mitre_tactics))
        .route("/mitre/techniques", post(get_mitre_techniques))
}
