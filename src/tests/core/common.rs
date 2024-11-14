use dotenv::dotenv;
use std::env;
use crate::client::WazuhClient;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct EnvVar(String);

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

lazy_static::lazy_static! {
    pub static ref WAZUH_URL: EnvVar = EnvVar(
        env::var("WAZUH_URL").expect("WAZUH_URL must be set in environment")
    );
    pub static ref PROXY_URL: EnvVar = EnvVar(
        env::var("PROXY_URL").expect("PROXY_URL must be set in environment")
    );
    pub static ref TEST_USERNAME: EnvVar = EnvVar(
        env::var("TEST_USERNAME").expect("TEST_USERNAME must be set in environment")
    );
    pub static ref TEST_PASSWORD: EnvVar = EnvVar(
        env::var("TEST_PASSWORD").expect("TEST_PASSWORD must be set in environment")
    );
    pub static ref TEST_AGENT_ID: EnvVar = EnvVar(
        env::var("TEST_AGENT_ID").expect("TEST_AGENT_ID must be set in environment")
    );
    pub static ref TEST_GROUP_ID: EnvVar = EnvVar(
        env::var("TEST_GROUP_ID").expect("TEST_GROUP_ID must be set in environment")
    );
}

pub async fn get_test_client() -> (WazuhClient, String) {
    // Ensure .env is loaded
    dotenv().ok();
    
    let client = WazuhClient::new();
    let token = client.get_auth_token()
        .await
        .expect("Failed to get auth token");
    (client, token)
}

// Helper functions to get string references
pub fn wazuh_url() -> &'static str {
    &WAZUH_URL.0
}

pub fn proxy_url() -> &'static str {
    &PROXY_URL.0
}

pub fn test_username() -> &'static str {
    &TEST_USERNAME.0
}

pub fn test_password() -> &'static str {
    &TEST_PASSWORD.0
}

pub fn test_agent_id() -> &'static str {
    &TEST_AGENT_ID.0
}

pub fn test_group_id() -> &'static str {
    &TEST_GROUP_ID.0
}
