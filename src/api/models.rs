use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Basic pagination parameters
#[derive(Debug, Serialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Parameter for filtering by entity
#[derive(Debug, Serialize)]
pub struct EntityFilterParams {
    pub entity_id: Option<String>,
    pub entity_name: Option<String>,
    pub cik: Option<String>,
}

/// Search request parameters
#[derive(Debug, Serialize)]
pub struct SearchParams {
    pub taxonomy: String,
    pub concept_name: Option<String>,
    pub entity_id: Option<String>,
    pub fiscal_year: Option<u32>,
    pub fiscal_period: Option<String>,
    pub dimension_name: Option<String>,
    pub member_name: Option<String>,
    pub text_search: Option<String>,
    pub value_greater_than: Option<f64>,
    pub value_less_than: Option<f64>,
}

/// API Response wrapper
/// status and errors fields are required by the API but not currently used directly in our code
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
    pub errors: Option<Vec<String>>,
}

/// Authentication header
#[derive(Debug, Serialize)]
pub struct AuthHeader {
    pub api_key: String,
}

/// Optional query parameters
#[allow(dead_code)]
pub type QueryParams = HashMap<String, String>;