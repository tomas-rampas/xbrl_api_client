use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Taxonomy data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Taxonomy {
    pub name: String,
    pub description: String,
    pub version: String,
    pub documentation_url: Option<String>,
}

/// Concept data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub taxonomy: String,
    pub type_name: String,
    pub period_type: Option<String>,
    pub balance: Option<String>,
    pub standard_label: Option<String>,
    pub documentation: Option<String>,
    pub is_abstract: Option<bool>,
    pub is_nillable: Option<bool>,
    pub references: Option<Vec<Reference>>,
    pub dimensions: Option<Vec<ConceptDimension>>,
}

/// Concept reference
#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub name: String,
    pub value: String,
}

/// Concept dimension
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptDimension {
    pub dimension_name: String,
    pub members: Vec<String>,
}

/// Dimension data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub taxonomy: String,
    pub members: Option<Vec<DimensionMember>>,
}

/// Dimension member
#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionMember {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
}

/// Network data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub taxonomy: String,
    pub role: String,
    pub nodes: Option<Vec<NetworkNode>>,
}

/// Network node
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkNode {
    pub concept_name: String,
    pub concept_label: String,
    pub parent: Option<String>,
    pub order: Option<f64>,
    pub level: Option<u32>,
    pub preferred_label: Option<String>,
    pub children: Option<Vec<NetworkNode>>,
}