use serde::{Deserialize, Serialize};
use super::models::GroupResponse;
use reqwest;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportSummary {
    pub total_agents: i32,
    pub total_alerts: i32,
    pub critical_vulnerabilities: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub success: bool,
    pub filename: String,
    pub pdf_data: String,
    pub summary: ReportSummary,
}

#[derive(Debug, Serialize)]
struct GenerateReportRequest {
    group_name: String,
    wql_data: GroupResponse,
}

pub async fn generate_report(group_response: GroupResponse) -> Result<Report, String> {
    let client = reqwest::Client::new();
    
    // Create the request in the format expected by the TypeScript service
    let report_request = GenerateReportRequest {
        group_name: group_response.group.clone(),
        wql_data: group_response,
    };

    // Send request to TypeScript service
    let response = client.post("http://localhost:3000/api/generate-report")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&report_request)
        .send()
        .await
        .map_err(|e| format!("Failed to send report generation request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(format!("Report generation failed: {}", error_text));
    }

    let mut report: Report = response.json().await
        .map_err(|e| format!("Failed to parse report response: {}", e))?;

    // Create reports directory if it doesn't exist
    let reports_dir = Path::new("reports");
    fs::create_dir_all(reports_dir)
        .map_err(|e| format!("Failed to create reports directory: {}", e))?;

    // Decode base64 PDF data
    let pdf_data = BASE64.decode(&report.pdf_data)
        .map_err(|e| format!("Failed to decode PDF data: {}", e))?;

    // Save PDF file
    let pdf_path = reports_dir.join(&report.filename);
    fs::write(&pdf_path, pdf_data)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    // Update the URL to point to the saved file
    report.pdf_data = String::new(); // Clear the base64 data as it's no longer needed
    
    Ok(report)
}
