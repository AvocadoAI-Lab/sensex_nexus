use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::error::Error;
use crate::tests::core::common::{get_test_client, proxy_url, wazuh_url};
use super::test_utils::{TestEndpoint, test_endpoint, setup_test_directory};

#[derive(Clone)]
pub struct TestFramework {
    pub client: Client,
    pub headers: HeaderMap,
    pub module_name: String,
    pub base_request: Value,
    proxy_url: String,
}

impl TestFramework {
    pub async fn new(module_name: &str) -> Result<Self, Box<dyn Error>> {
        // 設置測試目錄
        setup_test_directory(module_name)?;

        // 獲取認證 token
        let (_, token) = get_test_client().await;
        
        // 創建 HTTP client
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        // 創建 headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token))?);

        // 基本請求結構
        let base_request = serde_json::json!({
            "endpoint": wazuh_url(),
            "token": token,
            "url": wazuh_url()
        });

        Ok(Self {
            client,
            headers,
            module_name: module_name.to_string(),
            base_request,
            proxy_url: proxy_url().to_string(),
        })
    }

    pub async fn test_endpoint(&self, endpoint: TestEndpoint) -> Result<Value, Box<dyn Error>> {
        let max_retries = 3;
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match test_endpoint(
                &self.client,
                &self.headers,
                endpoint.clone(),
                &self.proxy_url,
                &self.module_name
            ).await {
                Ok(json_value) => return Ok(json_value),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        println!("Attempt {} failed, retrying...", attempt);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    pub fn create_endpoint(&self, path: &str) -> TestEndpoint {
        TestEndpoint::new(path, None, Some(self.base_request.clone()))
    }

    pub fn create_endpoint_with_params(&self, path: &str, param_desc: &str, params: Value) -> TestEndpoint {
        let mut request = self.base_request.clone();
        if let Value::Object(ref mut map) = request {
            map.insert("params".to_string(), params);
        }
        TestEndpoint::new(path, Some(param_desc), Some(request))
    }

    pub fn create_agent_endpoint(&self, path_template: &str, agent_id: &str) -> TestEndpoint {
        let path = path_template.replace("{agent_id}", agent_id);
        self.create_endpoint_with_params(
            &path,
            "agent_id",
            serde_json::json!({ "agent_id": agent_id })
        )
    }

    pub fn create_param_endpoint<T>(&self, path_template: &str, param_name: &str, param_value: T) -> TestEndpoint 
    where 
        T: serde::Serialize + std::fmt::Display,
    {
        let path = path_template.replace(&format!("{{{}}}", param_name), &param_value.to_string());
        self.create_endpoint_with_params(
            &path,
            param_name,
            serde_json::json!({ param_name: param_value })
        )
    }

    pub fn create_agent_config_endpoint(&self, agent_id: &str, config: &str) -> TestEndpoint {
        let path = format!("/agents/{}/config/agent/{}", agent_id, config);
        self.create_endpoint_with_params(
            &path,
            "agent_id, component, configuration",
            serde_json::json!({
                "agent_id": agent_id,
                "component": "agent",
                "configuration": config
            })
        )
    }

    pub fn create_agent_stats_endpoint(&self, agent_id: &str, component: &str) -> TestEndpoint {
        let path = format!("/agents/{}/stats/{}", agent_id, component);
        self.create_endpoint_with_params(
            &path,
            "agent_id, component",
            serde_json::json!({
                "agent_id": agent_id,
                "component": component
            })
        )
    }

    pub fn create_group_config_endpoint(&self, group_id: &str) -> TestEndpoint {
        let path = format!("/groups/{}/configuration", group_id);
        self.create_endpoint_with_params(
            &path,
            "group_id",
            serde_json::json!({ "group_id": group_id })
        )
    }

    pub fn create_group_files_endpoint(&self, group_id: &str) -> TestEndpoint {
        let path = format!("/groups/{}/files", group_id);
        self.create_endpoint_with_params(
            &path,
            "group_id",
            serde_json::json!({ "group_id": group_id })
        )
    }
}

// 基本端點宏
#[macro_export]
macro_rules! endpoints {
    ($framework:expr, $($path:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_endpoint($path),
            )*
        ]
    }};
}

// Agent端點宏
#[macro_export]
macro_rules! agent_endpoints {
    ($framework:expr, $agent_id:expr, $($path:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_agent_endpoint($path, $agent_id),
            )*
        ]
    }};
}

// 參數化端點宏
#[macro_export]
macro_rules! param_endpoints {
    ($framework:expr, $param_name:expr, $param_value:expr, $($path:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_param_endpoint($path, $param_name, $param_value),
            )*
        ]
    }};
}

// Agent配置端點宏
#[macro_export]
macro_rules! agent_config_endpoints {
    ($framework:expr, $agent_id:expr, $($config:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_agent_config_endpoint($agent_id, $config),
            )*
        ]
    }};
}

// Agent統計端點宏
#[macro_export]
macro_rules! agent_stats_endpoints {
    ($framework:expr, $agent_id:expr, $($component:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_agent_stats_endpoint($agent_id, $component),
            )*
        ]
    }};
}

// 組配置端點宏
#[macro_export]
macro_rules! group_config_endpoints {
    ($framework:expr, $($group_id:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_group_config_endpoint($group_id),
            )*
        ]
    }};
}

// 組文件端點宏
#[macro_export]
macro_rules! group_files_endpoints {
    ($framework:expr, $($group_id:expr),* $(,)?) => {{
        vec![
            $(
                $framework.create_group_files_endpoint($group_id),
            )*
        ]
    }};
}
