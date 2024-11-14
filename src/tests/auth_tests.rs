use crate::tests::core::common::{wazuh_url, test_username, test_password};
use crate::client::WazuhClient;

#[tokio::test]
async fn test_auth_flow() {
    // 1. 初始認證測試
    let client = WazuhClient::new();
    let auth_url = format!("{}/security/user/authenticate", wazuh_url());
    
    println!("Step 1: Testing initial authentication");
    let response = client
        .get_with_auth(&auth_url, test_username(), test_password())
        .await
        .expect("Authentication request failed");
    
    assert_eq!(response.status().as_u16(), 200, "Initial authentication should succeed");
    
    let text = response.text().await.expect("Failed to get response text");
    println!("Authentication Response: {}", text);
    
    // 2. 測試錯誤認證
    println!("\nStep 2: Testing failed authentication");
    let response = client
        .get_with_auth(&auth_url, test_username(), "wrong_password")
        .await
        .expect("Request should complete");
    
    assert_eq!(response.status().as_u16(), 401, "Should receive 401 for wrong password");
    
    // 3. 測試 token 獲取
    println!("\nStep 3: Testing token acquisition");
    let token = client.get_auth_token().await.expect("Should get valid token");
    assert!(!token.is_empty(), "Token should not be empty");
    println!("Successfully obtained token: {}", token);
    
    // 4. 使用 token 訪問受保護的端點
    println!("\nStep 4: Testing protected endpoint access with token");
    let test_url = format!("{}/agents", wazuh_url());
    let response = client.get(&test_url, Some(&token))
        .await
        .expect("Request with token should succeed");
    
    assert_eq!(response.status().as_u16(), 200, "Should access protected endpoint");
    let json = WazuhClient::handle_json_response(response).await.expect("Should get valid JSON");
    println!("Protected Endpoint Response: {:#?}", json);
}
