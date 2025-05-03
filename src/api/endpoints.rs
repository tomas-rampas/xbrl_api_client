/// Default base URL for the XBRL US API
pub const DEFAULT_API_BASE_URL: &str = "https://api.xbrl.us/api/v1";

/// API Endpoints
pub struct Endpoints {
    base_url: String,
}

impl Default for Endpoints {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_API_BASE_URL.to_string(),
        }
    }
}

impl Endpoints {
    /// Create a new Endpoints instance with a custom base URL
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
    
    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    
    /// Taxonomies endpoint
    pub fn taxonomies(&self) -> String {
        format!("{}/taxonomies", self.base_url)
    }
    
    /// Reports endpoint for a specific taxonomy
    pub fn reports(&self, taxonomy: &str) -> String {
        format!("{}/reports?taxonomy={}", self.base_url, taxonomy)
    }
    
    /// Facts endpoint for a specific report
    pub fn facts(&self, report_id: &str) -> String {
        format!("{}/facts?reportId={}", self.base_url, report_id)
    }
    
    /// Concepts endpoint for a specific taxonomy
    pub fn concepts(&self, taxonomy: &str) -> String {
        format!("{}/concepts?taxonomy={}", self.base_url, taxonomy)
    }
    
    /// Concept details endpoint
    pub fn concept_details(&self, taxonomy: &str, concept_name: &str) -> String {
        format!("{}/concepts/{}?taxonomy={}", self.base_url, concept_name, taxonomy)
    }
    
    /// Dimensions endpoint for a specific taxonomy
    pub fn dimensions(&self, taxonomy: &str) -> String {
        format!("{}/dimensions?taxonomy={}", self.base_url, taxonomy)
    }
    
    /// Networks endpoint for a specific taxonomy
    pub fn networks(&self, taxonomy: &str) -> String {
        format!("{}/networks?taxonomy={}", self.base_url, taxonomy)
    }
    
    /// Network details endpoint
    pub fn network_details(&self, network_id: &str) -> String {
        format!("{}/networks/{}", self.base_url, network_id)
    }
    
    /// Search endpoint
    pub fn search(&self) -> String {
        format!("{}/search", self.base_url)
    }
}