use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Syscollector information endpoints
        .route("/syscollector/:agent_id/hardware", post(get_syscollector_hardware))
        .route("/syscollector/:agent_id/hotfixes", post(get_syscollector_hotfixes))
        .route("/syscollector/:agent_id/netaddr", post(get_syscollector_netaddr))
        .route("/syscollector/:agent_id/netiface", post(get_syscollector_netiface))
        .route("/syscollector/:agent_id/netproto", post(get_syscollector_netproto))
        .route("/syscollector/:agent_id/os", post(get_syscollector_os))
        .route("/syscollector/:agent_id/packages", post(get_syscollector_packages))
        .route("/syscollector/:agent_id/ports", post(get_syscollector_ports))
        .route("/syscollector/:agent_id/processes", post(get_syscollector_processes))
}
