# Routes Directory

This directory defines the routing structure for the backend API, mapping URLs to their corresponding handlers.

## Authentication Flow

The routes follow a specific authentication pattern:
1. Client first calls POST `/auth` with credentials
2. Receives JWT token from Wazuh
3. Uses this JWT token in all subsequent POST requests
4. Our backend forwards these as GET requests to Wazuh with the JWT

## Architecture Overview

The routes in this directory establish POST endpoints that correspond to Wazuh's GET endpoints. This creates a proxy pattern where:
1. Clients POST to our routes with a JWT token
2. Our backend GETs from Wazuh using the provided JWT
3. Wazuh's response is returned to clients

## Key Components

- `auth.rs`: Defines the critical authentication endpoint
- `mod.rs`: Creates the main router and combines all subroutes
- Each route file (e.g., `agents.rs`, `groups.rs`) defines endpoints for a specific Wazuh feature

## Route Pattern

Each route file follows this pattern:
```rust
pub fn routes() -> Router {
    Router::new()
        .route("/endpoint", post(handler_function))
        .route("/endpoint/:param", post(handler_function))
}
```

Where:
- Routes match the Wazuh API structure but use POST instead of GET
- All routes except `/auth` require a JWT token in the request
- URL parameters are preserved to match Wazuh's API design

## Features

- Authentication route for obtaining JWT token
- Health check endpoint at `/health`
- Comprehensive coverage of Wazuh API endpoints
- All protected endpoints require valid JWT token
