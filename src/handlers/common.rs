use axum::Json;
use serde::Deserialize;
use crate::client::WazuhClient;

#[derive(Debug, Deserialize)]
pub struct WazuhRequest {
    pub endpoint: String,
    pub token: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
}

pub async fn handle_wazuh_request(request: WazuhRequest, url_path: &str, handler: impl FnOnce(String) -> String) -> Json<serde_json::Value> {
    let client = WazuhClient::new();
    
    // Replace URL parameters with actual values
    let mut final_path = url_path.to_string();
    for (key, value) in request.params.iter() {
        final_path = final_path.replace(&format!("{{{}}}", key), value);
    }
    
    let url = handler(format!("{}/{}", request.endpoint, final_path));
    
    println!("Proxying request to: {}", url);
    
    match client.get_cached(&url, Some(&request.token)).await {
        Ok(data) => {
            println!("Received response from Wazuh for {}", url);
            Json(data)
        },
        Err(e) => {
            println!("Error from Wazuh for {}: {}", url, e);
            Json(serde_json::json!({
                "error": e
            }))
        },
    }
}
