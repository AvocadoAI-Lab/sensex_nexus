use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(Clone)]
pub struct TestEndpoint {
    pub path: String,
    pub method: String,
    pub params: Option<String>,
    pub request_body: Option<Value>,
}

impl TestEndpoint {
    pub fn new(path: &str, params: Option<&str>, request_body: Option<Value>) -> Self {
        Self {
            path: path.to_string(),
            method: "GET".to_string(),
            params: params.map(String::from),
            request_body,
        }
    }
}

pub async fn test_endpoint(
    client: &Client,
    headers: &HeaderMap,
    endpoint: TestEndpoint,
    base_url: &str,
    module_name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    println!("\nTesting {} {}", endpoint.method, endpoint.path);
    
    let response = client
        .post(&format!("{}{}", base_url, endpoint.path))
        .headers(headers.clone())
        .json(&endpoint.request_body.clone().unwrap_or(serde_json::json!({})))
        .send()
        .await?;

    let status = response.status().as_u16();
    let text = response.text().await?;
    
    // Parse response text to JSON Value
    let json_value: Value = serde_json::from_str(&text)?;
    
    // Write test result and structure analysis
    if let Err(e) = write_test_results(&endpoint, status, &json_value, module_name).await {
        println!("Warning: Failed to write test results: {}", e);
    }
    
    Ok(json_value)
}

fn get_filename_with_params(path: &str, request_body: &Option<Value>) -> String {
    let mut filename = path.replace('/', "_").replace(':', "_").trim_start_matches('_').to_string();
    
    // Replace parameter placeholders with actual values
    if let Some(body) = request_body {
        if let Some(obj) = body.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{}}}", key);
                if let Some(value_str) = value.as_str() {
                    filename = filename.replace(&placeholder, value_str);
                }
            }
        }
    }
    
    filename
}

fn generate_rust_types(json_value: &Value, module_name: &str, endpoint_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create draft_models directory if it doesn't exist
    let models_dir = Path::new("draft_models");
    fs::create_dir_all(models_dir)?;

    // Generate a unique type name based on module and endpoint
    let type_name = format!("{}_{}",
        module_name,
        endpoint_path.replace('/', "_").replace(':', "_").trim_start_matches('_')
    ).replace(".", "_").replace("-", "_");

    // Create output path
    let output_path = models_dir.join(format!("{}.rs", type_name));

    // Convert JSON to string
    let json_str = serde_json::to_string_pretty(json_value)?;

    // Create json_typegen process with stdin pipe
    let mut child = Command::new("json_typegen")
        .arg("-n")
        .arg(&type_name)
        .arg("-")  // Use - to read from stdin
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write JSON to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(json_str.as_bytes())?;
    }

    // Get output
    let output = child.wait_with_output()?;

    if output.status.success() {
        // Add derive attributes and module declaration
        let generated_code = String::from_utf8(output.stdout)?;
        let final_code = format!(
            "#[allow(dead_code)]\n\
             use serde::{{Serialize, Deserialize}};\n\n\
             {}\n",
            generated_code
        );
        
        // Write the generated Rust code to file
        fs::write(output_path, final_code)?;
        Ok(())
    } else {
        Err(format!(
            "Failed to generate Rust types: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into())
    }
}

async fn write_test_results(
    endpoint: &TestEndpoint,
    status: u16,
    json_value: &Value,
    module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("test_results").join(module_name);
    let reports_dir = base_dir.join("reports");
    let raw_dir = base_dir.join("raw");

    // Create necessary directories
    fs::create_dir_all(&reports_dir)?;
    fs::create_dir_all(&raw_dir)?;

    // Get filename with parameters substituted
    let file_name = get_filename_with_params(&endpoint.path, &endpoint.request_body);
    
    // 寫入raw JSON到raw目錄
    let raw_path = raw_dir.join(format!("{}.json", file_name));
    let raw_json = serde_json::to_string_pretty(&json_value)?;
    fs::write(&raw_path, &raw_json)?;
    
    // Generate Rust types
    if let Err(e) = generate_rust_types(json_value, module_name, &endpoint.path) {
        println!("Warning: Failed to generate Rust types: {}", e);
    }
    
    // 寫入API響應結果到reports目錄
    let report_path = reports_dir.join(format!("{}.md", file_name));
    let pretty_json = serde_json::to_string_pretty(&json_value)?;
    let result_text = format!("# {} {}\n\n\
                              ## Status Code\n{}\n\n\
                              ## Parameters\n{}\n\n\
                              ## Response\n```json\n{}\n```\n",
        endpoint.method,
        endpoint.path,
        status,
        endpoint.params.as_deref().unwrap_or("無"),
        pretty_json
    );
    fs::write(&report_path, result_text)?;

    // 更新索引文件
    let index_path = base_dir.join("README.md");
    let mut index_content = String::new();
    index_content.push_str(&format!(
        "# {} Endpoints 測試結果\n\n\
         ## API 響應報告\n\n\
         以下是API響應的詳細報告：\n\n",
        module_name
    ));

    // Add raw section
    index_content.push_str("\n### Raw JSON\n\n");
    index_content.push_str(&format!("- [{}](./raw/{}.json)\n", endpoint.path, file_name));

    // Add reports section
    index_content.push_str("\n### 響應報告\n\n");
    index_content.push_str(&format!("- [{}](./reports/{}.md)\n", endpoint.path, file_name));

    // Write the complete content
    fs::write(&index_path, index_content)?;

    Ok(())
}

pub fn setup_test_directory(module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("test_results").join(module_name);
    let reports_dir = base_dir.join("reports");
    let raw_dir = base_dir.join("raw");

    // Remove existing directories if they exist
    if base_dir.exists() {
        fs::remove_dir_all(&base_dir)?;
    }

    // Create new directories
    fs::create_dir_all(&reports_dir)?;
    fs::create_dir_all(&raw_dir)?;
    
    // Create draft_models directory if it doesn't exist
    fs::create_dir_all("draft_models")?;
    
    Ok(())
}
