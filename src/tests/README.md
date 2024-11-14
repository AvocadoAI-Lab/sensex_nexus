# Tests Directory

This directory contains comprehensive test suites for validating the proxy functionality between client POST requests and Wazuh GET endpoints.

## Architecture Overview

The test suite is organized to mirror the Wazuh API structure, with each test file corresponding to a specific Wazuh feature. Tests verify that our proxy correctly:
1. Receives POST requests
2. Forwards them as GET requests to Wazuh
3. Returns the appropriate responses

## Key Components

### Core Testing Framework
- `TestFramework`: A custom testing utility that handles authentication and request validation
- Common test helpers and utilities for batch testing
- Shared constants and configurations

### Test Categories

#### Authentication Tests (`auth_tests.rs`)
- Tests the authentication flow
- Validates token acquisition and management
- Verifies error handling for invalid credentials

#### Agent Tests
- `agents_tests.rs`: Tests basic agent endpoints
- `agent_specific_tests.rs`: Tests agent-specific operations
- `syscollector_tests.rs`: Tests system information collection

#### Group Management Tests
- `groups_tests.rs`: Basic group operations
- `groups_with_agents_tests.rs`: Group-agent relationship testing

#### Security and Configuration Tests
- `security_tests.rs`: Security configuration testing
- `manager_tests.rs`: Manager configuration and status
- `rules_tests.rs`: Rule management testing
- `decoders_tests.rs`: Decoder configuration testing

## Test Patterns

### Endpoint Testing
```rust
let endpoints = endpoints!(framework,
    "/endpoint1",
    "/endpoint2"
);

for endpoint in endpoints {
    framework.test_endpoint(endpoint).await?;
}
```

### Batch Testing with Delays
```rust
batch_test_endpoints(&framework, endpoints, Some(500)).await;
```

### Retry Mechanism
- Implements automatic retries for flaky tests
- Configurable retry count and delay
- Detailed failure reporting

## Running Tests

Tests can be run using:
```bash
cargo test  # Run all tests
cargo test test_name  # Run specific test
```

## Test Results

Test results are stored in the `test_results/` directory, organized by feature:
- agent_specific/
- agents/
- decoders/
- groups/
- etc.

Each test generates detailed logs and response data for analysis.
