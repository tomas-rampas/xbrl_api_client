//! XBRL API Client for Rust
//! 
//! A Rust client library for interacting with the XBRL US API.
//! This library provides a simple and ergonomic way to fetch taxonomy, report,
//! and fact data from the XBRL API.

// Declare modules that are part of the public API
pub mod api;
pub mod data;
pub mod utils;

// Re-export commonly used types for convenience
pub use api::client::XbrlClient;
pub use api::models::SearchParams;
pub use data::facts::Fact;
pub use data::facts::FactValue;  // Add this line to explicitly re-export FactValue
pub use data::reports::Report;
pub use data::taxonomy::Taxonomy;
pub use utils::errors::{XbrlApiError, XbrlResult};