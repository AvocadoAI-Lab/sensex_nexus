use crate::tests::core::{
    TestFramework,
    common::test_agent_id,
    test_helpers::batch_test_endpoints,
};
use crate::{endpoints, agent_endpoints, agent_config_endpoints, agent_stats_endpoints};

const MODULE_NAME: &str = "agents";

#[tokio::test]
async fn test_agents_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // 基本端點測試
    let basic_endpoints = endpoints!(framework,
        "/agents",
        "/agents/no_group",
        "/agents/stats/distinct",
        "/agents/summary/os",
        "/agents/summary/status"
    );

    // Agent特定端點
    let agent_specific_endpoints = agent_endpoints!(framework, test_agent_id(),
        "/agents/{agent_id}/group/is_sync",
        "/agents/{agent_id}/daemons/stats"
    );

    // 配置端點測試 - 使用新的宏
    let config_endpoints = agent_config_endpoints!(framework, test_agent_id(),
        "buffer", "internal", "client", "labels"
    );

    // 統計端點測試 - 使用新的宏
    let stats_endpoints = agent_stats_endpoints!(framework, test_agent_id(),
        "logcollector", "agent"
    );

    // 批量測試所有端點
    for endpoint in basic_endpoints {
        framework.test_endpoint(endpoint).await?;
    }

    for endpoint in agent_specific_endpoints {
        framework.test_endpoint(endpoint).await?;
    }

    // 使用延遲的批量測試
    batch_test_endpoints(&framework, config_endpoints, Some(500)).await;
    batch_test_endpoints(&framework, stats_endpoints, Some(500)).await;

    Ok(())
}
