use reqwest::{Client, Response};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};
use dotenv::dotenv;

struct CacheEntry {
    data: Value,
    timestamp: Instant,
}

#[derive(Clone)]
pub struct WazuhClient {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    cache_duration: Duration,
}

impl WazuhClient {
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables from .env file
        
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        Self { 
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(300), // 5 minutes default cache
        }
    }

    pub async fn get_cached(&self, url: &str, token: Option<&str>) -> Result<Value, String> {
        let cache_key = match token {
            Some(t) => format!("{}:{}", url, t),
            None => url.to_string(),
        };

        // Try to get from cache first
        if let Some(cached_data) = self.get_from_cache(&cache_key).await {
            return Ok(cached_data);
        }

        // If not in cache, make the request
        let response = self.get(url, token).await
            .map_err(|e| format!("Request failed: {}", e))?;

        let data = Self::handle_json_response(response).await?;
        
        // Store in cache
        self.store_in_cache(&cache_key, data.clone()).await;
        
        Ok(data)
    }

    async fn get_from_cache(&self, key: &str) -> Option<Value> {
        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < self.cache_duration {
                return Some(entry.data.clone());
            }
        }
        None
    }

    async fn store_in_cache(&self, key: &str, data: Value) {
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), CacheEntry {
            data,
            timestamp: Instant::now(),
        });
    }

    pub async fn get(&self, url: &str, token: Option<&str>) -> Result<Response, reqwest::Error> {
        let mut request = self.client.get(url);
        
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        request.send().await
    }

    pub async fn get_with_auth(&self, url: &str, username: &str, password: &str) -> Result<Response, reqwest::Error> {
        self.client
            .get(url)
            .basic_auth(username, Some(password))
            .send()
            .await
    }

    pub async fn handle_json_response(response: Response) -> Result<Value, String> {
        match response.json::<Value>().await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        }
    }

    // This method is only used in tests
    #[cfg(test)]
    pub async fn get_auth_token(&self) -> Result<String, String> {
        dotenv().ok(); // Ensure environment variables are loaded in tests
        
        let wazuh_url = env::var("WAZUH_URL").expect("WAZUH_URL must be set");
        let username = env::var("WAZUH_USERNAME").expect("WAZUH_USERNAME must be set");
        let password = env::var("WAZUH_PASSWORD").expect("WAZUH_PASSWORD must be set");

        let auth_url = format!("{}/security/user/authenticate", wazuh_url);
        
        let response = self
            .get_with_auth(&auth_url, &username, &password)
            .await
            .map_err(|e| format!("Authentication request failed: {}", e))?;

        let json = Self::handle_json_response(response).await?;
        
        json.get("data")
            .and_then(|data| data.get("token"))
            .and_then(|token| token.as_str())
            .map(String::from)
            .ok_or_else(|| "Token not found in response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_cache_with_auth() {
        dotenv().ok(); // Load environment variables for tests
        let client = WazuhClient::new();
        let wazuh_url = env::var("WAZUH_URL").expect("WAZUH_URL must be set");
        let test_endpoint = format!("{}/security/user/authenticate", wazuh_url);
        let test_token = "test_token";
        
        // First request should hit the API
        let first_response = client
            .get_cached(&test_endpoint, Some(test_token))
            .await;
        assert!(first_response.is_ok(), "First request should succeed");
        
        // Second request should come from cache
        let second_response = client
            .get_cached(&test_endpoint, Some(test_token))
            .await;
        assert!(second_response.is_ok(), "Second request should succeed");
        
        assert_eq!(
            first_response.unwrap(),
            second_response.unwrap(),
            "Cached response should match original response"
        );
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let client = WazuhClient::new();
        let invalid_url = "https://invalid.example.com:55000";
        
        let response = client.get(invalid_url, None).await;
        assert!(response.is_err(), "Request should fail with invalid URL");
    }
}
