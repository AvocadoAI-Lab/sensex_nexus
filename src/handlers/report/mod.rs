use crate::handlers::wql::GroupResponse;
use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub url: String,
    pub summary: ReportSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_agents: i32,
    pub total_alerts: i32,
    pub critical_vulnerabilities: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateReportRequest {
    group_name: String,
    wql_data: GroupResponse,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateReportResponse {
    success: bool,
    url: String,
    summary: ReportSummary,
}

pub async fn generate_report(group_response: GroupResponse) -> Result<Report, String> {
    let client = reqwest::Client::new();
    
    let request_body = GenerateReportRequest {
        group_name: group_response.group.clone(),
        wql_data: group_response,
    };

    let response = client.post("http://localhost:3000/api/generate-report")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send report generation request: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Report generation failed: {}", error));
    }

    let report_response: GenerateReportResponse = response.json()
        .await
        .map_err(|e| format!("Failed to parse report response: {}", e))?;

    Ok(Report {
        url: report_response.url,
        summary: report_response.summary,
    })
}
