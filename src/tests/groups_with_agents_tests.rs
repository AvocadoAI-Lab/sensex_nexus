use crate::tests::core::{
    test_helpers::batch_test_endpoints,
    TestFramework,
};
use crate::param_endpoints;
use std::time::Instant;
use tokio::time::{sleep, Duration};

const MODULE_NAME: &str = "groups_with_agents";
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

// 新增: 用於創建組端點的宏
#[macro_export]
macro_rules! group_agents_endpoints {
    ($framework:expr, $groups:expr) => {{
        let mut endpoints = Vec::new();
        for group_name in $groups {
            endpoints.extend(param_endpoints!($framework, "group_id", group_name,
                "/groups/{group_id}/agents"
            ));
        }
        endpoints
    }};
}

async fn retry_test_endpoint(
    framework: &TestFramework,
    endpoint: crate::tests::core::test_utils::TestEndpoint,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match framework.test_endpoint(endpoint.clone()).await {
            Ok(response) => {
                return Ok(response);
            }
            Err(e) => {
                println!("Attempt {} failed: {}", attempt, e);
                last_error = Some(e);
                
                if attempt < MAX_RETRIES {
                    println!("Waiting {}ms before retry...", RETRY_DELAY_MS);
                    sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        }
    }

    Err(format!(
        "Failed after {} attempts over {:?}. Last error: {:?}",
        MAX_RETRIES,
        start.elapsed(),
        last_error
    ).into())
}

#[tokio::test]
async fn test_groups_with_agents() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // 獲取所有組
    println!("Fetching groups list...");
    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = retry_test_endpoint(&framework, groups_endpoint).await?;

    // 添加延遲
    sleep(Duration::from_millis(500)).await;

    // 獲取所有組名
    let mut group_names = Vec::new();
    if let Some(affected_items) = groups_response["data"]["affected_items"].as_array() {
        for group in affected_items {
            if let Some(group_name) = group["name"].as_str() {
                group_names.push(group_name.to_string());
            }
        }
    }

    // 使用新的宏創建所有組的端點
    let all_group_endpoints = group_agents_endpoints!(framework, group_names);

    // 批量測試所有端點
    batch_test_endpoints(&framework, all_group_endpoints, Some(500)).await;

    Ok(())
}
