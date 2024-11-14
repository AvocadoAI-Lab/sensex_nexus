use crate::tests::core::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "security";

#[tokio::test]
async fn test_security_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/security/actions",
        "/security/resources",
        "/security/config"
    );

    // Test each endpoint individually
    for endpoint in endpoints {
        framework.test_endpoint(endpoint).await?;
    }
    
    Ok(())
}
