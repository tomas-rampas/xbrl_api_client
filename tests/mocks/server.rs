use serde_json::Value;
use serde_json;  // No need for macro_use
use std::fs;
use std::path::Path;
use wiremock::{
    matchers::{method, path, path_regex, query_param},
    Mock, MockServer, ResponseTemplate,
};

pub struct MockXbrlServer {
    pub server: MockServer,
}

impl MockXbrlServer {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    pub fn url(&self) -> String {
        self.server.uri()
    }

    pub async fn mock_taxonomies(&mut self) {
        let response_body = load_mock_data("taxonomies.json");
        
        // Add debug info
        println!("Setting up mock at URL: {}", self.url());
        
        // Need to wrap the response in the API response format
        let api_response = serde_json::json!({
            "status": "success",
            "message": null,
            "data": response_body,
            "errors": null
        });
        
        // Make sure the path exactly matches what the client is requesting via the endpoints
        Mock::given(method("GET"))
            .and(path("/taxonomies"))  // Use exact path without api/v1
            .respond_with(ResponseTemplate::new(200).set_body_json(api_response))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_reports(&mut self, taxonomy: &str) {
        let response_body = load_mock_data("reports.json");
        
        // This file is already in the API response format, so use it directly
        Mock::given(method("GET"))
            .and(path("/reports"))
            .and(query_param("taxonomy", taxonomy))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_facts(&mut self, report_id: &str) {
        let response_body = load_mock_data("facts.json");
        
        // This file is already in the API response format, so use it directly
        Mock::given(method("GET"))
            .and(path("/facts"))
            .and(query_param("reportId", report_id))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_concept_details(&mut self, taxonomy: &str, concept_name: &str) {
        let response_body = load_mock_data("concept_details.json");
        
        // This file is already in the API response format, so use it directly
        Mock::given(method("GET"))
            .and(path(format!("/concepts/{}", concept_name)))
            .and(query_param("taxonomy", taxonomy))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_search(&mut self) {
        let response_body = load_mock_data("search_results.json");
        
        // This file is already in the API response format, so use it directly
        Mock::given(method("POST"))
            .and(path("/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }

    pub async fn mock_unauthorized(&mut self) {
        Mock::given(method("GET"))
            .and(path("/unauthorized"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({
                        "status": "error",
                        "message": "Unauthorized. Invalid API key.",
                        "data": null,
                        "errors": ["Invalid API key"]
                    }))
            )
            .mount(&self.server)
            .await;
    }

    pub async fn mock_server_error(&mut self) {
        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({
                        "status": "error",
                        "message": "Internal server error",
                        "data": null,
                        "errors": ["Something went wrong on the server"]
                    }))
            )
           .mount(&self.server)
            .await;
    }
    
    pub async fn mock_concepts(&mut self, taxonomy: &str) {
        let response_body = load_mock_data("concepts.json");
        
        Mock::given(method("GET"))
            .and(path("/concepts"))
            .and(query_param("taxonomy", taxonomy))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }
    
    pub async fn mock_dimensions(&mut self, taxonomy: &str) {
        let response_body = load_mock_data("dimensions.json");
        
        Mock::given(method("GET"))
            .and(path("/dimensions"))
            .and(query_param("taxonomy", taxonomy))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }
    
    pub async fn mock_networks(&mut self, taxonomy: &str) {
        let response_body = load_mock_data("networks.json");
        
        Mock::given(method("GET"))
            .and(path("/networks"))
            .and(query_param("taxonomy", taxonomy))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }
    
    pub async fn mock_network_details(&mut self, network_id: &str) {
        let response_body = load_mock_data("network_details.json");
        
        Mock::given(method("GET"))
            .and(path(format!("/networks/{}", network_id)))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&self.server)
            .await;
    }
}

fn load_mock_data(filename: &str) -> Value {
    let path = Path::new("tests/mock_data").join(filename);
    let data = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read mock data file: {}", filename));
    
    serde_json::from_str(&data)
        .unwrap_or_else(|_| panic!("Failed to parse mock data file: {}", filename))
}
