use axum::{
    Router,
    routing::post,
};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        // Base API info
        .route("/", post(get_api_info))
        
        // Manager status and info
        .route("/manager/status", post(get_manager_status))
        .route("/manager/info", post(get_manager_info))
        .route("/manager/configuration", post(get_manager_configuration))
        
        // Statistics endpoints
        .route("/manager/stats", post(get_manager_stats))
        .route("/manager/stats/hourly", post(get_manager_hourly_stats))
        .route("/manager/stats/weekly", post(get_manager_weekly_stats))
        
        // Logs endpoints
        .route("/manager/logs", post(get_manager_logs))
        .route("/manager/logs/summary", post(get_manager_logs_summary))
}
