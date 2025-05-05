use crate::api::endpoints::Endpoints;
use crate::api::models::{ApiResponse, PaginationParams, QueryParams, SearchParams};
use crate::data::facts::Fact;
use crate::data::reports::Report;
use crate::data::taxonomy::{Concept, Dimension, Network, Taxonomy};
use crate::utils::errors::{XbrlApiError, XbrlResult};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
// HashMap is used in the with_pagination method through QueryParams

/// XBRL API Client
pub struct XbrlClient {
    client: Client,
    api_key: String,
    endpoints: Endpoints,
}

#[allow(dead_code)]
impl XbrlClient {
    /// Create a new XBRL API client with the default base URL
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            endpoints: Endpoints::default(),
        }
    }
    
    /// Create a new XBRL API client with a custom base URL
    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            endpoints: Endpoints::new(base_url),
        }
    }
    
    /// Set the base URL for the API endpoints
    pub fn set_base_url(&mut self, base_url: &str) {
        self.endpoints = Endpoints::new(base_url);
    }
    
    /// Get the current base URL
    pub fn base_url(&self) -> &str {
        self.endpoints.base_url()
    }
    
    /// Add authentication header to request
    fn auth_request(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("X-API-KEY", &self.api_key)
    }
    
    /// Execute API request and parse response
    pub async fn execute_request<T>(&self, request: RequestBuilder) -> XbrlResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self.auth_request(request).send().await?;
        
        let status_code = response.status().as_u16();
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(XbrlApiError::ApiError {
                status_code,
                message: error_text,
            });
        }
        
        let api_response: ApiResponse<T> = response.json().await?;
        
        match api_response.data {
            Some(data) => Ok(data),
            None => Err(XbrlApiError::ApiError {
                status_code,
                message: api_response.message.unwrap_or_else(|| "No error message provided".into()),
            }),
        }
    }
    
    // Get all taxonomies
    pub async fn get_taxonomies(&self) -> Result<Vec<Taxonomy>, XbrlApiError> {
        // Use the endpoints struct for consistency with other methods
        let taxonomies_url = self.endpoints.taxonomies();
        println!("Requesting taxonomies from URL: {}", taxonomies_url);
        
        let request = self.client.get(&taxonomies_url);
        
        self.execute_request::<Vec<Taxonomy>>(request).await
    }
    
    /// Get reports for a specific taxonomy
    pub async fn get_reports(&self, taxonomy: &str) -> XbrlResult<Vec<Report>> {
        let request = self.client.get(self.endpoints.reports(taxonomy));
        self.execute_request(request).await
    }
    
    /// Get facts for a specific report
    pub async fn get_facts(&self, report_id: &str) -> XbrlResult<Vec<Fact>> {
        let request = self.client.get(self.endpoints.facts(report_id));
        self.execute_request(request).await
    }
    
    /// Get concepts for a specific taxonomy
    pub async fn get_concepts(&self, taxonomy: &str) -> XbrlResult<Vec<Concept>> {
        let request = self.client.get(self.endpoints.concepts(taxonomy));
        self.execute_request(request).await
    }
    
    /// Get details for a specific concept
    pub async fn get_concept_details(&self, taxonomy: &str, concept_name: &str) -> XbrlResult<Concept> {
        let request = self.client.get(self.endpoints.concept_details(taxonomy, concept_name));
        self.execute_request(request).await
    }
    
    /// Get dimensions for a specific taxonomy
    pub async fn get_dimensions(&self, taxonomy: &str) -> XbrlResult<Vec<Dimension>> {
        let request = self.client.get(self.endpoints.dimensions(taxonomy));
        self.execute_request(request).await
    }
    
    /// Get networks for a specific taxonomy
    pub async fn get_networks(&self, taxonomy: &str) -> XbrlResult<Vec<Network>> {
        let request = self.client.get(self.endpoints.networks(taxonomy));
        self.execute_request(request).await
    }
    
    /// Get details for a specific network
    pub async fn get_network_details(&self, network_id: &str) -> XbrlResult<Network> {
        let request = self.client.get(self.endpoints.network_details(network_id));
        self.execute_request(request).await
    }
    
    /// Search for facts
    pub async fn search(&self, params: SearchParams) -> XbrlResult<Vec<Fact>> {
        let request = self.client
            .post(self.endpoints.search())
            .json(&params);
        self.execute_request(request).await
    }
    
    /// Utility method to build queries with pagination
    pub fn with_pagination(params: &mut QueryParams, pagination: Option<PaginationParams>) {
        if let Some(pagination) = pagination {
            if let Some(page) = pagination.page {
                params.insert("page".to_string(), page.to_string());
            }
            if let Some(page_size) = pagination.page_size {
                params.insert("pageSize".to_string(), page_size.to_string());
            }
        }
    }
}
