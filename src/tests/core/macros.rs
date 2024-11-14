//! Test macros for simplifying test creation and execution

/// Creates a test function with proper async setup and error handling
#[macro_export]
macro_rules! wazuh_test {
    ($name:ident, $module:expr, $test_fn:expr) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let framework = TestFramework::new($module).await?;
            $test_fn(&framework).await
        }
    };
}

/// Creates a test function with retries
#[macro_export]
macro_rules! wazuh_test_with_retry {
    ($name:ident, $module:expr, $test_fn:expr, $max_retries:expr, $delay_ms:expr) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use tokio::time::{sleep, Duration};
            
            let framework = TestFramework::new($module).await?;
            let mut last_error = None;
            
            for attempt in 1..=$max_retries {
                match $test_fn(&framework).await {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        println!("Attempt {} failed: {}", attempt, e);
                        last_error = Some(e);
                        
                        if attempt < $max_retries {
                            sleep(Duration::from_millis($delay_ms)).await;
                        }
                    }
                }
            }
            
            Err(format!(
                "Failed after {} attempts. Last error: {:?}",
                $max_retries,
                last_error
            ).into())
        }
    };
}

/// Creates a test suite for multiple endpoints
#[macro_export]
macro_rules! test_endpoints {
    ($framework:expr, $($path:expr),* $(,)?) => {{
        let mut results = Vec::new();
        $(
            match $framework.test_endpoint($framework.create_endpoint($path)).await {
                Ok(response) => {
                    println!("Successfully tested {}", $path);
                    results.push(Ok(response));
                }
                Err(e) => {
                    println!("Failed to test {}: {}", $path, e);
                    results.push(Err(e));
                }
            }
        )*
        results
    }};
}

/// Creates a test suite for endpoints with parameters
#[macro_export]
macro_rules! test_endpoints_with_params {
    ($framework:expr, $(($path:expr, $desc:expr, $params:expr)),* $(,)?) => {{
        let mut results = Vec::new();
        $(
            match $framework.test_endpoint(
                $framework.create_endpoint_with_params($path, $desc, $params)
            ).await {
                Ok(response) => {
                    println!("Successfully tested {}", $path);
                    results.push(Ok(response));
                }
                Err(e) => {
                    println!("Failed to test {}: {}", $path, e);
                    results.push(Err(e));
                }
            }
        )*
        results
    }};
}

/// Asserts that all test results are successful
#[macro_export]
macro_rules! assert_all_success {
    ($results:expr) => {
        for result in $results {
            assert!(result.is_ok(), "Test failed: {:?}", result.err().unwrap());
        }
    };
}
