# Client Directory

This directory contains the `WazuhClient` implementation that handles communication with the Wazuh server.

## Authentication Flow

The WazuhClient implements a critical authentication process:
1. Client calls `get_with_auth()` with username/password
2. Wazuh returns a JWT token
3. This JWT token is required for all subsequent requests
4. Client uses `get()` or `get_cached()` with the JWT token

## Architecture Overview

The `WazuhClient` implements a caching proxy pattern where:
1. Our backend receives POST requests with JWT from clients
2. We check our cache for a recent response
3. If not cached, we forward the request as GET to Wazuh with the JWT
4. Responses are cached for future use

## Key Features

### Authentication
- Handles initial JWT token acquisition
- Manages JWT token in subsequent requests
- Provides methods for both authenticated and unauthenticated requests

### Caching
- Responses are cached for 5 minutes by default
- Cache keys combine URL and JWT token for security
- Automatic cache invalidation based on time

### Request Handling
- Supports authenticated requests with JWT tokens
- Handles HTTPS with self-signed certificates
- Provides methods for both cached and direct requests

## Usage Example

```rust
let client = WazuhClient::new();

// First authenticate to get JWT
let auth_response = client.get_with_auth(auth_url, username, password).await?;
let jwt_token = extract_token(auth_response);

// Then use JWT for subsequent requests
let response = client.get_cached(url, Some(jwt_token)).await?;
```

This client manages the entire authentication flow and subsequent request handling while providing caching capabilities.
