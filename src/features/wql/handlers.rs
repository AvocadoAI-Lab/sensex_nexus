use axum::Json;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use dotenv::dotenv;
use native_tls::TlsConnector as NativeTlsConnector;
use reqwest;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;
use uuid::Uuid;
use tokio::time::timeout;

use crate::shared::common::WazuhRequest;
use super::{models::*, report};

const SERVER_ADDR: &str = "172.104.127.21:8080";
const CLIENT_ID: &str = "client1";
const CLIENT_KEY: &str = "test_key_1";
const SERVER_KEY: &str = "server_key";
const INITIAL_BUFFER_SIZE: usize = 8192;
const MAX_BUFFER_SIZE: usize = 1024 * 1024 * 10; // 10MB
const CHUNK_SIZE: usize = 1024 * 64; // Reduced to 64KB chunks for better stability
const REQUEST_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes timeout
const MAX_RETRIES: u32 = 5; // Increased retries
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);

fn get_template_path(report_type: &ReportType) -> &'static str {
    match report_type {
        ReportType::Daily => "wql_templates/alerts_daily.json",
        ReportType::Weekly => "wql_templates/alerts_weekly.json",
        ReportType::Monthly => "wql_templates/alerts_monthly.json",
    }
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
    let mut response_data = Vec::with_capacity(INITIAL_BUFFER_SIZE);
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_bytes = 0;
    let mut last_read = SystemTime::now();

    loop {
        match timeout(Duration::from_secs(60), stream.read(&mut buffer)).await {
            Ok(read_result) => {
                match read_result {
                    Ok(0) => {
                        if total_bytes == 0 {
                            return Err("Connection closed by server".to_string());
                        }
                        break;
                    },
                    Ok(n) => {
                        if total_bytes + n > MAX_BUFFER_SIZE {
                            return Err("Response too large".to_string());
                        }
                        response_data.extend_from_slice(&buffer[..n]);
                        total_bytes += n;
                        last_read = SystemTime::now();

                        // Send keepalive if needed
                        if SystemTime::now().duration_since(last_read).unwrap() >= KEEPALIVE_INTERVAL {
                            stream.write_all(b"\n").await
                                .map_err(|e| format!("Failed to send keepalive: {}", e))?;
                        }
                    }
                    Err(e) => return Err(format!("Failed to read response: {}", e)),
                }
            },
            Err(_) => return Err("Read timeout".to_string()),
        }
    }

    String::from_utf8(response_data)
        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

async fn establish_connection() -> Result<tokio_native_tls::TlsStream<TcpStream>, String> {
    // Connect to server with timeout
    let stream = match timeout(Duration::from_secs(30), TcpStream::connect(SERVER_ADDR)).await {
        Ok(result) => result.map_err(|e| e.to_string())?,
        Err(_) => return Err("Connection timeout".to_string()),
    };
    
    // Configure TCP stream
    stream.set_nodelay(true)
        .map_err(|e| format!("Failed to set TCP_NODELAY: {}", e))?;
    
    // Set keepalive
    stream.set_keepalive(Some(Duration::from_secs(30)))
        .map_err(|e| format!("Failed to set keepalive: {}", e))?;

    // Initialize TLS connector with custom configuration
    let mut connector = NativeTlsConnector::builder();
    connector.danger_accept_invalid_certs(true);
    let connector = connector.build()
        .map_err(|e| e.to_string())?;
    let connector = TlsConnector::from(connector);

    connector.connect("localhost", stream)
        .await
        .map_err(|e| e.to_string())
}

async fn send_request_with_retry(wql_query: String) -> Result<Response, String> {
    let mut retries = 0;
    let mut last_error = String::new();

    loop {
        // Establish new connection for each retry
        match establish_connection().await {
            Ok(mut stream) => {
                match send_request(&mut stream, wql_query.clone()).await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        last_error = e.to_string();
                        if retries >= MAX_RETRIES {
                            return Err(format!("Max retries exceeded. Last error: {}", last_error));
                        }
                    }
                }
            },
            Err(e) => {
                last_error = e.to_string();
                if retries >= MAX_RETRIES {
                    return Err(format!("Max retries exceeded. Last error: {}", last_error));
                }
            }
        }

        retries += 1;
        println!("Request failed, retrying ({}/{}): {}", retries, MAX_RETRIES, last_error);
        
        // Exponential backoff with jitter
        let backoff = 2u64.pow(retries) + (rand::random::<u64>() % 1000);
        tokio::time::sleep(Duration::from_millis(backoff)).await;
    }
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

    // Split request into chunks if it's large
    let chunks: Vec<&[u8]> = request_json.as_bytes()
        .chunks(CHUNK_SIZE)
        .collect();

    // Send request with timeout
    match timeout(REQUEST_TIMEOUT, async {
        for chunk in chunks {
            stream.write_all(chunk).await?;
            stream.flush().await?;
            // Small delay between chunks
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<(), std::io::Error>(())
    }).await {
        Ok(result) => result.map_err(|e| e.to_string())?,
        Err(_) => return Err("Request timeout".to_string()),
    }

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

fn load_query_template(report_type: &ReportType) -> Result<Value, String> {
    let template_path = get_template_path(report_type);
    let template_str = fs::read_to_string(template_path)
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
                    match_obj["match"]["agent.name"] = Value::String(agent_name.to_string());
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
    let response = client.post("http://localhost:29000/auth")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&auth_request)
        .send()
        .await
        .map_err(|e| format!("Failed to send auth request: {}", e))?;

    let status = response.status();
    let body = response.text()
        .await
        .map_err(|e| format!("Failed to read auth response: {}", e))?;

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

    let response = crate::shared::common::handle_wazuh_request(request, "groups/{group_id}/agents", |url| url).await;

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
    group: String,
    report_type: ReportType,
) -> Result<Json<QueryResponse>, String> {
    println!("Starting WQL query for group: {} with report type: {:?}", group, report_type);
    
    // First authenticate with Wazuh
    let token = authenticate().await?;
    println!("Authentication successful");
    
    // Load query template based on report type
    let template = load_query_template(&report_type)?;
    println!("Query template loaded");
    
    // Get all agents in the group using Wazuh API
    let agents = get_agents_in_group(&group, &token).await?;
    println!("Found {} agents in group {}", agents.len(), group);
    
    let mut results = Vec::new();
    let mut total_alerts = 0;

    // Execute query for each agent
    for agent in agents {
        println!("Processing agent: {}", agent.name);
        
        // Prepare query with agent name
        let wql_query = prepare_query(&template, &agent.name)?;

        // Send request with retry mechanism and new connection for each agent
        let response = send_request_with_retry(wql_query).await?;
        
        // Parse response data
        let data: Value = serde_json::from_str(&response.data)
            .map_err(|e| format!("Failed to parse response data: {}", e))?;

        // Count alerts from the response
        if let Some(total) = data["hits"]["total"]["value"].as_i64() {
            total_alerts += total;
            println!("Found {} alerts for agent {}", total, agent.name);
        }

        results.push(AgentResult {
            agent_name: agent.name,
            data,
        });

        // Add small delay between agents to prevent overwhelming the server
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    let group_response = GroupResponse {
        group: group.clone(),
        results,
    };

    // Generate report using the TypeScript service
    println!("Generating report for group: {}", group);
    let report = match report::generate_report(group_response.clone()).await {
        Ok(r) => {
            println!("Report generated successfully");
            r
        },
        Err(e) => {
            println!("Error generating report: {}", e);
            return Err(format!("Failed to generate report: {}", e));
        }
    };

    // Create a simplified response for the API
    let response = QueryResponse {
        raw_data: group_response,
        report,
    };

    println!("Query completed successfully for group: {}", group);
    Ok(Json(response))
}
