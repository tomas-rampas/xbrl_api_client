# XBRL API Client for Rust

A Rust client library for interacting with the XBRL US API. This library provides a comprehensive set of methods to fetch taxonomies, reports, facts, and other XBRL-related data.

[![Build Status](https://github.com/tomas-rampas/xbrl_api_client/actions/workflows/rust.yml/badge.svg)](https://github.com/tomas-rampas/xbrl_api_client/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/tomas-rampas/xbrl_api_client/branch/main/graph/badge.svg)](https://codecov.io/gh/tomas-rampas/xbrl_api_client)
[![Test Coverage](https://raw.githubusercontent.com/tomas-rampas/xbrl_api_client/master/.github/badges/coverage.svg?raw=true)](https://github.com/tomas-rampas/xbrl_api_client/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Fetch taxonomies, reports, and facts from the XBRL US API
- Retrieve detailed concept information
- Search for facts using flexible filters
- Robust error handling
- Comprehensive test suite with mock server for integration testing

## Project Structure

```
xbrl_api_client/
├── src/
│   ├── main.rs              # Entry point for CLI application
│   ├── lib.rs               # Library exports
│   ├── api/
│   │   ├── mod.rs           # API module exports
│   │   ├── client.rs        # XBRL API client implementation
│   │   ├── endpoints.rs     # API endpoint definitions
│   │   └── models.rs        # Data models for API requests/responses
│   ├── data/
│   │   ├── mod.rs           # Data module exports
│   │   ├── facts.rs         # Fact data structures
│   │   ├── reports.rs       # Report data structures
│   │   └── taxonomy.rs      # Taxonomy data structures
│   └── utils/
│       ├── mod.rs           # Utils module exports
│       └── errors.rs        # Error handling
└── tests/
    ├── integration_tests.rs # Integration tests with mock server
    ├── unit_tests.rs        # Unit tests for library components
    ├── library_test.rs      # Integration tests with real API (optional)
    ├── mock_data/           # Mock data for tests
    │   ├── taxonomies.json
    │   ├── reports.json
    │   ├── facts.json
    │   ├── concept_details.json
    │   └── search_results.json
    └── mocks/               # Mock server implementation
        ├── mod.rs
        └── server.rs
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xbrl_api_client = "0.1.0"
```

### Setting Up Environment Variables

The XBRL API Client requires an API key to authenticate with the XBRL US API. You have two options for providing this key:

1. **Environment Variable**: Set the `XBRL_API_KEY` environment variable directly in your system
   ```bash
   # On Linux/macOS
   export XBRL_API_KEY=your_api_key_here
   
   # On Windows (Command Prompt)
   set XBRL_API_KEY=your_api_key_here
   
   # On Windows (PowerShell)
   $env:XBRL_API_KEY = "your_api_key_here"
   ```

2. **Using a .env File**: Create a `.env` file in the root of your project with the following content:
   ```
   XBRL_API_KEY=your_api_key_here
   ```
   
   The library will automatically load this file if present, making it easier to manage environment variables during development.

## Usage

### Basic Example

```rust
use dotenv::dotenv;
use xbrl_api_client::api::client::XbrlClient;
use xbrl_api_client::api::models::SearchParams;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenv().ok();
    
    // Initialize client with API key
    let api_key = env::var("XBRL_API_KEY").expect("XBRL_API_KEY must be set in environment or .env file");
    let client = XbrlClient::new(&api_key);
    
    // Get all taxonomies
    let taxonomies = client.get_taxonomies().await?;
    println!("Available taxonomies:");
    for taxonomy in &taxonomies {
        println!("- {} ({})", taxonomy.name, taxonomy.version);
    }
    
    // Get reports for a specific taxonomy
    let taxonomy_name = "us-gaap";
    let reports = client.get_reports(taxonomy_name).await?;
    println!("\nFound {} reports for taxonomy {}", reports.len(), taxonomy_name);
    
    // Get facts for a specific report
    if let Some(first_report) = reports.first() {
        println!("\nFetching facts for report: {}", first_report.id);
        let facts = client.get_facts(&first_report.id).await?;
        println!("Found {} facts", facts.len());
        
        // Print first 5 facts
        for (i, fact) in facts.iter().take(5).enumerate() {
            println!("Fact {}: {} = {:?}", i + 1, fact.concept_name, fact.value);
        }
    }
    
    Ok(())
}
```

### Advanced Usage: Searching for Facts

```rust
use dotenv::dotenv;
use xbrl_api_client::api::client::XbrlClient;
use xbrl_api_client::api::models::SearchParams;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenv().ok();
    
    let api_key = env::var("XBRL_API_KEY").expect("XBRL_API_KEY must be set in environment or .env file");
    let client = XbrlClient::new(&api_key);
    
    // Search for Assets with value over $1M
    let search_params = SearchParams {
        taxonomy: "us-gaap".to_string(),
        concept_name: Some("Assets".to_string()),
        entity_id: None,
        fiscal_year: Some(2023),
        fiscal_period: Some("FY".to_string()),
        dimension_name: None,
        member_name: None,
        text_search: None,
        value_greater_than: Some(1_000_000.0),
        value_less_than: None,
    };
    
    let search_results = client.search(search_params).await?;
    println!("Search results: {} facts found", search_results.len());
    
    Ok(())
}
```

## API Reference

### Client Methods

| Method | Description |
|--------|-------------|
| `new(api_key: &str)` | Create new client with default base URL |
| `with_base_url(api_key: &str, base_url: &str)` | Create client with custom base URL |
| `set_base_url(&mut self, base_url: &str)` | Update the base URL |
| `get_taxonomies()` | Get list of available taxonomies |
| `get_reports(taxonomy: &str)` | Get reports for specific taxonomy |
| `get_facts(report_id: &str)` | Get facts for specific report |
| `get_concept_details(taxonomy: &str, concept_name: &str)` | Get detailed info about a concept |
| `search(params: SearchParams)` | Search for facts using filters |

### Data Structures

The library provides strongly-typed structs for all API responses, including:

- `Taxonomy`: Information about a taxonomy
- `Report`: Filing report data
- `Fact`: Detailed fact information
- `Concept`: Concept metadata and details
- `FactValue`: Enum representing different value types (String, Number, Boolean)

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run unit tests
cargo test --test unit_tests

# Run integration tests (mock server)
cargo test --test integration_tests

# Run library tests with real API (requires API key)
# You can either set the environment variable as shown below
# or create a .env file with XBRL_API_KEY=your_api_key_here
export XBRL_API_KEY=your_api_key_here
cargo test --test library_test -- --ignored
```

### Integration Test Notes

The integration tests use a mock server to simulate the XBRL API. If you encounter issues with the integration tests, check:

1. The test context setup to ensure the mock server and client are properly configured
2. The mock data files to ensure they match the expected API response format
3. The path patterns in the mock server to ensure they match what the client is requesting

A common issue is path mismatches between what the client is requesting and what the mock server is registered to handle. The updated tests use a `TestContext` struct to ensure the same mock server instance is used consistently.

### Code Coverage

The project uses [grcov](https://github.com/mozilla/grcov) for code coverage analysis in CI. Coverage reports are automatically generated by GitHub Actions and uploaded to [Codecov](https://codecov.io/gh/tomas-rampas/xbrl_api_client).

You can run code coverage locally using either grcov or cargo-tarpaulin:

#### Using grcov (same as CI)

```bash
# Install grcov if not already installed
cargo install grcov

# Run tests with coverage instrumentation
CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw" cargo test

# Generate lcov report
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o coverage/lcov.info
```

#### Using cargo-tarpaulin (alternative)

```bash
# Install cargo-tarpaulin if not already installed
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --verbose --workspace --timeout 120
```

Either tool will generate a code coverage report. For HTML reports, you can convert lcov output to HTML with tools like genhtml.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
