pub mod summary;
pub mod template;
pub mod pdf;

use crate::handlers::wql::GroupResponse;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub summary: summary::Summary,
    pub html_content: String,
    pub pdf_path: Option<String>,
}

pub async fn generate_report(group_response: GroupResponse) -> Result<Report, String> {
    // Generate summary
    let summary = summary::generate_summary(&group_response)?;
    
    // Generate HTML content using template
    let html_content = template::generate_html(&group_response, &summary)?;
    
    // Generate PDF
    let pdf_path = pdf::generate_pdf(&html_content).await?;
    
    Ok(Report {
        summary,
        html_content,
        pdf_path: Some(pdf_path),
    })
}
