use crate::tests::core::{
    TestFramework,
    common::test_group_id,
    test_helpers::batch_test_endpoints,
};
use crate::{endpoints, group_config_endpoints, group_files_endpoints};

const MODULE_NAME: &str = "groups";

#[tokio::test]
async fn test_groups_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    // 基本端點
    let basic_endpoints = endpoints!(framework,
        "/groups"
    );

    // 組特定端點
    let group_files = group_files_endpoints!(framework, test_group_id());
    let group_configs = group_config_endpoints!(framework, test_group_id());

    // 批量測試所有端點
    for endpoint in basic_endpoints {
        framework.test_endpoint(endpoint).await?;
    }

    batch_test_endpoints(&framework, group_files, Some(500)).await;
    batch_test_endpoints(&framework, group_configs, Some(500)).await;

    Ok(())
}
