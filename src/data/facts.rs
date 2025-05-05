use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fact data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    pub id: String,
    pub concept_name: String,
    pub concept_label: Option<String>,
    pub entity_id: String,
    pub entity_name: Option<String>,
    pub period_start: Option<String>,
    pub period_end: String,
    pub value: FactValue,
    pub unit: Option<String>,
    pub dimensions: Option<HashMap<String, String>>,
    pub report_id: String,
    pub filing_url: Option<String>,
}

/// Fact value can be a string, number, or boolean
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FactValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Fact context
#[derive(Debug, Serialize, Deserialize)]
pub struct FactContext {
    pub entity_id: String,
    pub entity_name: Option<String>,
    pub period_start: Option<String>,
    pub period_end: String,
    pub dimensions: HashMap<String, String>,
}