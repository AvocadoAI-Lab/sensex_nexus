# Handlers Directory

This directory contains the handler functions that process incoming POST requests and forward them to Wazuh as GET requests.

## Authentication Flow

Before any endpoint can be accessed, a JWT token must be obtained from Wazuh:
1. Client makes a POST request to `/auth` with username and password
2. Our backend forwards this to Wazuh's `/security/user/authenticate` endpoint
3. Wazuh returns a JWT token
4. This JWT token must be included in all subsequent requests

## Architecture Overview

The handlers in this directory implement a proxy pattern where:
1. Clients make POST requests to our backend with the JWT token
2. Our backend forwards these as GET requests to Wazuh, including the JWT
3. Wazuh's responses are cached and returned to clients

## Key Components

- `auth.rs`: Handles the critical JWT authentication with Wazuh
- `common.rs`: Contains the core `WazuhRequest` struct that includes the JWT token
- Other handlers for various Wazuh features (agents, groups, etc.)

## Handler Pattern

Each handler follows this general pattern:
```rust
pub async fn get_something(Json(payload): Json<WazuhRequest>) -> Json<serde_json::Value> {
    // payload.token contains the JWT from Wazuh
    handle_wazuh_request(payload, "endpoint/path", |url| url).await
}
```

Where:
- `payload`: Contains the client's request data including endpoint and JWT token
- The second parameter is the Wazuh API endpoint path
- The closure modifies the URL if needed before the request is made

## Security Note

The JWT token is required for all endpoints except `/auth`. Any request without a valid JWT token will be rejected by Wazuh. The token is automatically included in the Authorization header when forwarding requests to Wazuh.
