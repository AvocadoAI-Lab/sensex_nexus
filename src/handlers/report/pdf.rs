use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;
use std::fs;
use serde_json::json;
use crate::handlers::wql::GroupResponse;
use super::summary::Summary;

pub async fn generate_pdf(group_response: &GroupResponse, summary: &Summary) -> Result<String, String> {
    println!("Starting PDF generation process");

    // Create a unique report ID
    let report_id = Uuid::new_v4().to_string();
    
    // Create reports directory if it doesn't exist
    let reports_dir = PathBuf::from("reports");
    fs::create_dir_all(&reports_dir)
        .map_err(|e| format!("Failed to create reports directory: {}", e))?;

    // Create JSON data for Python script
    let json_data = json!({
        "group": group_response.group,
        "total_alerts": summary.total_alerts,
        "time_analysis": {
            "alerts_by_hour": summary.time_analysis.alerts_by_hour,
            "alerts_by_day": summary.time_analysis.alerts_by_day,
            "first_alert": summary.time_analysis.first_alert.map(|dt| dt.to_rfc3339()),
            "last_alert": summary.time_analysis.last_alert.map(|dt| dt.to_rfc3339())
        },
        "alerts_by_level": summary.alerts_by_level,
        "alerts_by_category": summary.alerts_by_category,
        "alerts_by_mitre": summary.alerts_by_mitre,
        "agents_overview": summary.agents_overview.iter().map(|agent| {
            json!({
                "name": agent.name,
                "total_alerts": agent.total_alerts,
                "highest_level": agent.highest_level,
                "alert_distribution": agent.alert_distribution,
                "categories": agent.categories,
                "last_alert": agent.last_alert.map(|dt| dt.to_rfc3339())
            })
        }).collect::<Vec<_>>()
    });

    // Write JSON to temporary file
    let json_path = reports_dir.join(format!("{}.json", report_id));
    fs::write(&json_path, json_data.to_string())
        .map_err(|e| format!("Failed to write JSON data: {}", e))?;

    // Define output PDF path
    let pdf_filename = format!("report_{}.pdf", report_id);
    let pdf_path = reports_dir.join(&pdf_filename);

    // Run Python script using virtual environment
    println!("Running Python script to generate PDF");
    let python_executable = if cfg!(windows) {
        ".venv\\Scripts\\python.exe"
    } else {
        ".venv/bin/python"
    };

    let output = Command::new(python_executable)
        .arg("scripts/generate_report.py")
        .arg(&json_path)
        .arg(&pdf_path)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    // Check if the script executed successfully
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python script failed: {}", error));
    }

    // Clean up temporary JSON file
    if let Err(e) = fs::remove_file(&json_path) {
        println!("Warning: Failed to remove temporary JSON file: {}", e);
    }

    // Verify PDF was created
    if !pdf_path.exists() {
        return Err("PDF file was not generated".to_string());
    }

    println!("PDF generated successfully: {:?}", pdf_path);
    Ok(pdf_filename)
}
