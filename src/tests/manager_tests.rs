use crate::tests::core::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "manager";

#[tokio::test]
async fn test_manager_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        // Base API info
        "/",
        
        // Manager status and info
        "/manager/status",
        "/manager/info",
        "/manager/configuration",
        
        // Statistics endpoints
        "/manager/stats",
        "/manager/stats/hourly",
        "/manager/stats/weekly",
        
        // Logs endpoints
        "/manager/logs",
        "/manager/logs/summary"
    );

    // Test each endpoint individually
    for endpoint in endpoints {
        framework.test_endpoint(endpoint).await?;
    }
    
    Ok(())
}
