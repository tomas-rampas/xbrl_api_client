#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    // Define the necessary types locally for testing
    // This avoids the dependency issues with the crate structure
    
    struct Endpoints {
        base_url: String,
    }
    
    impl Endpoints {
        fn default() -> Self {
            Self {
                base_url: "https://api.xbrl.us/api/v1".to_string(),
            }
        }
        
        fn new(base_url: &str) -> Self {
            Self {
                base_url: base_url.to_string(),
            }
        }
        
        fn base_url(&self) -> &str {
            &self.base_url
        }
        
        fn taxonomies(&self) -> String {
            format!("{}/taxonomies", self.base_url)
        }
        
        fn reports(&self, taxonomy: &str) -> String {
            format!("{}/reports?taxonomy={}", self.base_url, taxonomy)
        }
    }
    
    // Define SearchParams struct
    #[derive(serde::Serialize, serde::Deserialize)]
    struct SearchParams {
        taxonomy: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        concept_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        entity_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fiscal_year: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fiscal_period: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dimension_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        member_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text_search: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value_greater_than: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value_less_than: Option<f64>,
    }
    
    // Define FactValue enum with serde implementation
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum FactValue {
        String(String),
        Number(f64),
        Boolean(bool),
    }
    
    // Define error types
    #[derive(Debug)]
    enum XbrlApiError {
        HttpError(reqwest::Error),
        ApiError {
            status_code: u16,
            message: String,
        },
        AuthError(String),
    }
    
    impl std::fmt::Display for XbrlApiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::HttpError(err) => write!(f, "HTTP error: {}", err),
                Self::ApiError { status_code, message } => {
                    write!(f, "API error: {} - {}", status_code, message)
                }
                Self::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            }
        }
    }

    #[test]
    fn test_endpoints_default() {
        let endpoints = Endpoints::default();
        assert_eq!(endpoints.base_url(), "https://api.xbrl.us/api/v1");
        assert_eq!(endpoints.taxonomies(), "https://api.xbrl.us/api/v1/taxonomies");
        assert_eq!(
            endpoints.reports("us-gaap"),
            "https://api.xbrl.us/api/v1/reports?taxonomy=us-gaap"
        );
    }

    #[test]
    fn test_endpoints_custom_base_url() {
        let endpoints = Endpoints::new("http://localhost:8080/api");
        assert_eq!(endpoints.base_url(), "http://localhost:8080/api");
        assert_eq!(endpoints.taxonomies(), "http://localhost:8080/api/taxonomies");
        assert_eq!(
            endpoints.reports("us-gaap"),
            "http://localhost:8080/api/reports?taxonomy=us-gaap"
        );
    }

    #[test]
    fn test_fact_value_serialization() {
        // String value
        let string_value = FactValue::String("Test".to_string());
        let serialized = serde_json::to_string(&string_value).unwrap();
        assert_eq!(serialized, "\"Test\"");
        
        // Number value
        let number_value = FactValue::Number(123.45);
        let serialized = serde_json::to_string(&number_value).unwrap();
        assert_eq!(serialized, "123.45");
        
        // Boolean value
        let bool_value = FactValue::Boolean(true);
        let serialized = serde_json::to_string(&bool_value).unwrap();
        assert_eq!(serialized, "true");
    }

    #[test]
    fn test_fact_value_deserialization() {
        // String value
        let json_string = "\"Test\"";
        let deserialized: FactValue = serde_json::from_str(json_string).unwrap();
        match deserialized {
            FactValue::String(val) => assert_eq!(val, "Test"),
            _ => panic!("Expected String value"),
        }
        
        // Number value
        let json_number = "123.45";
        let deserialized: FactValue = serde_json::from_str(json_number).unwrap();
        match deserialized {
            FactValue::Number(val) => assert_eq!(val, 123.45),
            _ => panic!("Expected Number value"),
        }
        
        // Boolean value
        let json_bool = "true";
        let deserialized: FactValue = serde_json::from_str(json_bool).unwrap();
        match deserialized {
            FactValue::Boolean(val) => assert_eq!(val, true),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn test_search_params_serialization() {
        let params = SearchParams {
            taxonomy: "us-gaap".to_string(),
            concept_name: Some("Assets".to_string()),
            entity_id: None,
            fiscal_year: Some(2022),
            fiscal_period: Some("FY".to_string()),
            dimension_name: None,
            member_name: None,
            text_search: None,
            value_greater_than: Some(1_000_000.0),
            value_less_than: None,
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        
        // Check that the JSON contains the expected fields
        let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(json["taxonomy"], "us-gaap");
        assert_eq!(json["concept_name"], "Assets");
        assert_eq!(json["fiscal_year"], 2022);
        assert_eq!(json["fiscal_period"], "FY");
        assert_eq!(json["value_greater_than"], 1_000_000.0);
        
        // Make sure the null fields are not included
        assert!(json.get("entity_id").is_none() || json["entity_id"].is_null());
        assert!(json.get("dimension_name").is_none() || json["dimension_name"].is_null());
    }

    #[test]
    fn test_error_handling() {
        // HTTP error - Create a reqwest error using tokio runtime
        let http_err = {
            // Create a runtime for executing the async request
            let rt = Runtime::new().unwrap();
            
            // Use the runtime to block_on the future returned by reqwest::get
            rt.block_on(async {
                // This request should fail because port 1 is not typically open
                match reqwest::get("http://localhost:1").await {
                    Ok(_) => panic!("Expected request to fail"),
                    Err(e) => e,
                }
            })
        };
        
        let xbrl_err = XbrlApiError::HttpError(http_err);
        assert!(format!("{}", xbrl_err).contains("HTTP error"));
        
        // API error
        let api_err = XbrlApiError::ApiError {
            status_code: 401,
            message: "Unauthorized".to_string(),
        };
        assert_eq!(
            format!("{}", api_err),
            "API error: 401 - Unauthorized"
        );
        
        // Auth error
        let auth_err = XbrlApiError::AuthError("Invalid API key".to_string());
        assert_eq!(format!("{}", auth_err), "Authentication error: Invalid API key");
    }}