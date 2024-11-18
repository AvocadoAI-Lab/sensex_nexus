use axum::{
    Router,
    routing::{post, get},
    extract::{Path as AxumPath, Query},
    response::{IntoResponse, Response},
    http::{header, StatusCode},
};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use super::handlers::handle_wql_query;
use tokio::fs;

type HeaderPair = [(header::HeaderName, &'static str); 2];
type ApiResponse = (StatusCode, HeaderPair, Vec<u8>);

#[derive(Debug, Deserialize)]
struct WqlQuery {
    format: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/wql/:group", post(handle_wql_query_wrapper))
        .route("/reports/:filename", get(serve_pdf))
}

async fn serve_pdf(AxumPath(filename): AxumPath<String>) -> ApiResponse {
    let pdf_path = PathBuf::from("reports").join(filename);
    
    match fs::read(&pdf_path).await {
        Ok(content) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/pdf"),
                (header::CONTENT_DISPOSITION, "inline"),
            ],
            content
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            [
                (header::CONTENT_TYPE, "text/plain"),
                (header::CONTENT_DISPOSITION, "inline"),
            ],
            b"PDF not found".to_vec()
        ),
    }
}

async fn handle_wql_query_wrapper(
    AxumPath(group): AxumPath<String>,
    Query(params): Query<WqlQuery>,
) -> ApiResponse {
    // Call the original handler
    match handle_wql_query(group).await {
        Ok(full_response) => {
            // Check if PDF format was requested
            if params.format.as_deref() == Some("pdf") {
                let pdf_path = PathBuf::from("reports").join(&full_response.0.report.filename);
                match fs::read(&pdf_path).await {
                    Ok(content) => (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, "application/pdf"),
                            (header::CONTENT_DISPOSITION, "inline"),
                        ],
                        content
                    ),
                    Err(_) => (
                        StatusCode::NOT_FOUND,
                        [
                            (header::CONTENT_TYPE, "text/plain"),
                            (header::CONTENT_DISPOSITION, "inline"),
                        ],
                        b"PDF not found".to_vec()
                    ),
                }
            } else {
                // Return JSON response with PDF URL
                let pdf_url = format!("http://localhost:3001/reports/{}", full_response.0.report.filename);
                
                let response = json!({
                    "status": "success",
                    "group": full_response.0.raw_data.group,
                    "total_agents": full_response.0.raw_data.results.len(),
                    "report_file": full_response.0.report.filename,
                    "pdf_url": pdf_url,
                    "note": "To get PDF directly, add ?format=pdf to the URL"
                });

                (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "application/json"),
                        (header::CONTENT_DISPOSITION, "inline"),
                    ],
                    response.to_string().into_bytes()
                )
            }
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [
                (header::CONTENT_TYPE, "text/plain"),
                (header::CONTENT_DISPOSITION, "inline"),
            ],
            e.into_bytes()
        ),
    }
}
