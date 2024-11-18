use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::report::Report;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub client_id: String,
    pub timestamp: u64,
    pub nonce: String,
    pub signature: String,
    pub session_id: Option<String>,
    pub wql_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WazuhAuthRequest {
    pub endpoint: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WazuhAuthResponse {
    pub token: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub status: bool,
    pub data: String,
    pub session_id: String,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupResponse {
    pub group: String,
    pub results: Vec<AgentResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentResult {
    pub agent_name: String,
    pub data: Value,
}

// New simplified response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SimplifiedQueryResponse {
    pub group: String,
    pub total_agents: i32,
    pub total_alerts: i32,
    pub report_file: String,
}

// Keep the full response for internal use
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub raw_data: GroupResponse,
    pub report: Report,
}
