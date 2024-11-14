use crate::tests::core::{
    TestFramework,
    common::test_agent_id,
    test_helpers::batch_test_endpoints,
};
use crate::agent_endpoints;

const MODULE_NAME: &str = "syscollector";

#[tokio::test]
async fn test_syscollector_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let syscollector_endpoints = agent_endpoints!(framework, test_agent_id(),
        "/syscollector/{agent_id}/hardware",
        "/syscollector/{agent_id}/hotfixes",
        "/syscollector/{agent_id}/netaddr",
        "/syscollector/{agent_id}/netiface",
        "/syscollector/{agent_id}/netproto",
        "/syscollector/{agent_id}/os",
        "/syscollector/{agent_id}/packages",
        "/syscollector/{agent_id}/ports",
        "/syscollector/{agent_id}/processes"
    );

    // 批量測試所有端點
    batch_test_endpoints(&framework, syscollector_endpoints, Some(500)).await;

    Ok(())
}
