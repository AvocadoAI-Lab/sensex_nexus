use axum::{
    Json,
    extract::Path,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;
use native_tls::TlsConnector as NativeTlsConnector;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use crate::handlers::common::{WazuhRequest, handle_wazuh_request};
use crate::client::WazuhClient;
use reqwest;

const SERVER_ADDR: &str = "172.104.127.21:8080";
const CLIENT_ID: &str = "client1";
const CLIENT_KEY: &str = "test_key_1";
const SERVER_KEY: &str = "server_key";
const BUFFER_SIZE: usize = 8192;
const QUERY_TEMPLATE_PATH: &str = "wql_queries/alerts.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRequest {
    client_id: String,
    timestamp: u64,
    nonce: String,
    signature: String,
    session_id: Option<String>,
    wql_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WazuhAuthRequest {
    endpoint: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WazuhAuthResponse {
    token: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub status: bool,
    pub data: String,
    pub session_id: String,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupResponse {
    pub group: String,
    pub results: Vec<AgentResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResult {
    pub agent_name: String,
    pub data: Value,
}

fn sign_request(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.update(CLIENT_KEY.as_bytes());
    BASE64.encode(hasher.finalize())
}

fn verify_response(response_data: &str, signature: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(response_data.as_bytes());
    hasher.update(SERVER_KEY.as_bytes());
    let expected = BASE64.encode(hasher.finalize());
    expected == signature
}

async fn stream_response(stream: &mut tokio_native_tls::TlsStream<TcpStream>) -> Result<String, String> {
    let mut response_data = Vec::new();
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut total_bytes = 0;

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                if total_bytes == 0 {
                    return Err("Connection closed by server".to_string());
                }
                break;
            },
            Ok(n) => {
                response_data.extend_from_slice(&buffer[..n]);
                total_bytes += n;
            }
            Err(e) => return Err(format!("Failed to read response: {}", e)),
        }
    }

    String::from_utf8(response_data)
        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

async fn send_request(stream: &mut tokio_native_tls::TlsStream<TcpStream>, wql_query: String) -> Result<Response, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    
    let nonce = Uuid::new_v4().to_string();
    
    let data_to_sign = format!("{}:{}:{}", 
        CLIENT_ID,
        timestamp,
        nonce
    );

    let signature = sign_request(&data_to_sign);

    let request = AuthRequest {
        client_id: CLIENT_ID.to_string(),
        timestamp,
        nonce,
        signature,
        session_id: None,
        wql_query,
    };

    let request_json = serde_json::to_string(&request)
        .map_err(|e| e.to_string())?;

    stream.write_all(request_json.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    
    stream.flush()
        .await
        .map_err(|e| e.to_string())?;

    let response_str = stream_response(stream).await?;
    
    let mut response: Response = serde_json::from_str(&response_str)
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    let signature = response.signature.clone();
    response.signature = String::new();
    let response_data = serde_json::to_string(&response)
        .map_err(|e| format!("Failed to serialize response: {}", e))?;
    
    if !verify_response(&response_data, &signature) {
        return Err("Invalid response signature".to_string());
    }

    response.signature = signature;
    Ok(response)
}

fn load_query_template() -> Result<Value, String> {
    let template_str = fs::read_to_string(QUERY_TEMPLATE_PATH)
        .map_err(|e| format!("Failed to read query template: {}", e))?;
    
    serde_json::from_str(&template_str)
        .map_err(|e| format!("Failed to parse query template: {}", e))
}

fn prepare_query(template: &Value, agent_name: &str) -> Result<String, String> {
    let mut query = template.clone();
    
    // Replace {{agent_name}} placeholder
    if let Some(must) = query["query"]["bool"]["must"].as_array_mut() {
        if let Some(match_obj) = must.get_mut(0) {
            if let Some(name) = match_obj["match"]["agent.name"].as_str() {
                if name == "{{agent_name}}" {
                    match_obj["match"]["agent.name"] = serde_json::Value::String(agent_name.to_string());
                }
            }
        }
    }

    serde_json::to_string(&query)
        .map_err(|e| format!("Failed to serialize query: {}", e))
}

async fn authenticate() -> Result<String, String> {
    dotenv().ok();

    let wazuh_url = env::var("WAZUH_URL")
        .map_err(|_| "WAZUH_URL must be set in .env file".to_string())?;
    let wazuh_username = env::var("WAZUH_USERNAME")
        .map_err(|_| "WAZUH_USERNAME must be set in .env file".to_string())?;
    let wazuh_password = env::var("WAZUH_PASSWORD")
        .map_err(|_| "WAZUH_PASSWORD must be set in .env file".to_string())?;

    let auth_request = WazuhAuthRequest {
        endpoint: wazuh_url.clone(),
        username: wazuh_username,
        password: wazuh_password,
    };

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3001/auth")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&auth_request)
        .send()
        .await
        .map_err(|e| format!("Failed to send auth request: {}", e))?;

    let status = response.status();
    let body = response.text()
        .await
        .map_err(|e| format!("Failed to read auth response: {}", e))?;

    println!("Auth response status: {}", status);
    println!("Auth response body: {}", body);

    if status.is_success() {
        let auth_response: WazuhAuthResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse auth response: {}", e))?;
        
        auth_response.token
            .ok_or_else(|| "Authentication failed: No token received".to_string())
    } else {
        Err(format!("Authentication failed: {}", body))
    }
}

async fn get_agents_in_group(group: &str, token: &str) -> Result<Vec<Agent>, String> {
    dotenv().ok();

    let wazuh_url = env::var("WAZUH_URL")
        .map_err(|_| "WAZUH_URL must be set in .env file".to_string())?;

    let mut params = HashMap::new();
    params.insert("group_id".to_string(), group.to_string());

    let request = WazuhRequest {
        endpoint: wazuh_url,
        token: token.to_string(),
        params,
    };

    let response = handle_wazuh_request(request, "groups/{group_id}/agents", |url| url).await;
    
    println!("Wazuh response: {}", serde_json::to_string_pretty(&response.0).unwrap());

    let agents = match &response.0 {
        value => {
            let mut agents = Vec::new();
            if let Some(items) = value.get("data")
                .and_then(|d| d.get("affected_items"))
                .and_then(|i| i.as_array()) {
                for item in items {
                    if let (Some(id), Some(name)) = (item.get("id").and_then(|i| i.as_str()), 
                                                    item.get("name").and_then(|n| n.as_str())) {
                        agents.push(Agent {
                            id: id.to_string(),
                            name: name.to_string(),
                        });
                    }
                }
            }
            if agents.is_empty() {
                return Err(format!("No agents found in group {}", group));
            }
            agents
        }
    };

    Ok(agents)
}

pub async fn handle_wql_query(
    Path(group): Path<String>,
) -> Result<Json<GroupResponse>, String> {
    // First authenticate with Wazuh
    let token = authenticate().await?;
    
    // Load query template
    let template = load_query_template()?;
    
    // Get all agents in the group using Wazuh API
    let agents = get_agents_in_group(&group, &token).await?;
    
    let mut results = Vec::new();

    // Initialize TLS connector
    let connector = NativeTlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;
    let connector = TlsConnector::from(connector);

    // Execute query for each agent
    for agent in agents {
        println!("Querying agent: {}", agent.name);
        
        // Prepare query with agent name
        let wql_query = prepare_query(&template, &agent.name)?;

        // Connect to server
        let stream = TcpStream::connect(SERVER_ADDR)
            .await
            .map_err(|e| e.to_string())?;
        
        let mut stream = connector
            .connect("localhost", stream)
            .await
            .map_err(|e| e.to_string())?;

        // Send request and get response
        let response = send_request(&mut stream, wql_query).await?;
        
        // Parse response data
        let data: Value = serde_json::from_str(&response.data)
            .map_err(|e| format!("Failed to parse response data: {}", e))?;

        results.push(AgentResult {
            agent_name: agent.name,
            data,
        });
    }
    
    Ok(Json(GroupResponse {
        group,
        results,
    }))
}
