use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::client::WazuhClient;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub endpoint: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: Option<String>,
    pub error: Option<String>,
}

pub async fn authenticate(Json(payload): Json<AuthRequest>) -> (StatusCode, Json<AuthResponse>) {
    let client = WazuhClient::new();
    let auth_url = format!("{}/security/user/authenticate", payload.endpoint);
    
    match client.get_with_auth(&auth_url, &payload.username, &payload.password).await {
        Ok(response) => {
            if response.status().as_u16() == 401 {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse {
                        token: None,
                        error: Some("Invalid credentials".to_string()),
                    })
                );
            }

            match WazuhClient::handle_json_response(response).await {
                Ok(data) => {
                    if let Some(token) = data["data"]["token"].as_str() {
                        (
                            StatusCode::OK,
                            Json(AuthResponse {
                                token: Some(token.to_string()),
                                error: None,
                            })
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(AuthResponse {
                                token: None,
                                error: Some("Token not found in response".to_string()),
                            })
                        )
                    }
                },
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(AuthResponse {
                        token: None,
                        error: Some(e),
                    })
                ),
            }
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                token: None,
                error: Some(format!("Request failed: {}", e)),
            })
        ),
    }
}
