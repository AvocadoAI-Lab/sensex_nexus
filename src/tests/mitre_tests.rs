use crate::tests::core::test_framework::TestFramework;
use crate::endpoints;

const MODULE_NAME: &str = "mitre";

#[tokio::test]
async fn test_mitre_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let framework = TestFramework::new(MODULE_NAME).await?;

    let endpoints = endpoints!(framework,
        "/mitre/groups",
        "/mitre/metadata",
        "/mitre/mitigations",
        "/mitre/references",
        "/mitre/software",
        "/mitre/tactics",
        "/mitre/techniques"
    );

    // Test each endpoint individually
    for endpoint in endpoints {
        framework.test_endpoint(endpoint).await?;
    }
    
    Ok(())
}
