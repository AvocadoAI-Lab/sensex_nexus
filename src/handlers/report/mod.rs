pub mod summary;
pub mod pdf;

use crate::handlers::wql::GroupResponse;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub summary: summary::Summary,
    pub pdf_path: Option<String>,
}

fn check_python_env() -> Result<(), String> {
    let venv_path = Path::new(".venv");
    if !venv_path.exists() {
        // Try to run setup script
        println!("Python environment not found. Running setup script...");
        let output = Command::new("cmd")
            .args(["/C", "scripts\\setup.bat"])
            .output()
            .map_err(|e| format!("Failed to run setup script: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "Failed to set up Python environment. Please run scripts/setup.bat manually.\nError: {}", 
                error
            ));
        }
    }
    Ok(())
}

pub async fn generate_report(group_response: GroupResponse) -> Result<Report, String> {
    // Ensure Python environment is properly set up
    check_python_env()?;

    // Generate summary
    let summary = summary::generate_summary(&group_response)?;
    
    // Generate PDF
    let pdf_path = pdf::generate_pdf(&group_response, &summary).await?;
    
    Ok(Report {
        summary,
        pdf_path: Some(pdf_path),
    })
}
