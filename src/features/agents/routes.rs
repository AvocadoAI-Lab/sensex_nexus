use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Base agents endpoints
        .route("/agents", post(get_agents))
        
        // Agent configuration and stats
        .route("/agents/:agent_id/config/:component/:configuration", post(get_agent_config_by_id))
        .route("/agents/:agent_id/group/is_sync", post(get_agent_group_sync_status))
        .route("/agents/:agent_id/daemons/stats", post(get_daemon_stats))
        .route("/agents/:agent_id/stats/:component", post(get_agent_stats_component))
        
        // Group related endpoints
        .route("/agents/no_group", post(get_agents_without_group))
        
        // Status and summary endpoints
        .route("/agents/outdated", post(get_outdated_agents))
        .route("/agents/stats/distinct", post(get_distinct_agents_stats))
        .route("/agents/summary/os", post(get_agents_os_summary))
        .route("/agents/summary/status", post(get_agents_status_summary))
}
