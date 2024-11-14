use crate::tests::core::{
    TestFramework,
    common::test_agent_id,
    test_helpers::batch_test_endpoints,
};
use crate::{agent_endpoints, agent_config_endpoints, agent_stats_endpoints};

const MODULE_NAME: &str = "agent_specific";

pub async fn test_agent_specific_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // 配置端點測試 - 使用新的宏
    let config_endpoints = agent_config_endpoints!(framework, test_agent_id(),
        "buffer", "internal", "client", "labels"
    );

    // 統計端點測試 - 使用新的宏
    let stats_endpoints = agent_stats_endpoints!(framework, test_agent_id(),
        "logcollector", "agent"
    );

    // Syscollector端點
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

    // 使用延遲的批量測試
    batch_test_endpoints(&framework, config_endpoints, Some(500)).await;
    batch_test_endpoints(&framework, stats_endpoints, Some(500)).await;
    batch_test_endpoints(&framework, syscollector_endpoints, Some(500)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_specific() {
        test_agent_specific_endpoints().await.unwrap();
    }
}
