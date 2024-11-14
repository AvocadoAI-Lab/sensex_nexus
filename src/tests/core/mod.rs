pub mod common;
pub mod test_framework;
pub mod test_utils;
pub mod macros;

// Re-export commonly used items
pub use test_framework::TestFramework;
pub use test_utils::TestEndpoint;

// Helper functions for test organization
pub mod test_helpers {
    use super::*;
    use std::error::Error;
    use std::fmt::Debug;
    use tokio::time::{sleep, Duration};

    pub async fn retry_operation<F, T, E>(
        operation: F,
        max_retries: u32,
        delay_ms: u64,
    ) -> Result<T, Box<dyn Error>>
    where
        F: Fn() -> Result<T, E>,
        E: Into<Box<dyn Error>> + Debug,
    {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("Attempt {} failed", attempt);
                    last_error = Some(e);
                    
                    if attempt < max_retries {
                        sleep(Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }
        
        Err(format!(
            "Failed after {} attempts. Last error: {:?}",
            max_retries,
            last_error
        ).into())
    }

    pub async fn batch_test_endpoints(
        framework: &TestFramework,
        endpoints: Vec<TestEndpoint>,
        delay_ms: Option<u64>,
    ) -> Vec<Result<serde_json::Value, Box<dyn Error>>> {
        let mut results = Vec::new();
        
        for endpoint in endpoints {
            let result = framework.test_endpoint(endpoint.clone()).await;
            results.push(result);
            
            if let Some(delay) = delay_ms {
                sleep(Duration::from_millis(delay)).await;
            }
        }
        
        results
    }

    pub fn validate_response(
        response: &serde_json::Value,
        expected_fields: &[&str],
    ) -> Result<(), Box<dyn Error>> {
        for field in expected_fields {
            if !response.get(field).is_some() {
                return Err(format!("Missing expected field: {}", field).into());
            }
        }
        Ok(())
    }
}
