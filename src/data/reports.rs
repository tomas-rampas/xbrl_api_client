use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Report data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub accession_number: Option<String>,
    pub filing_date: String,
    pub fiscal_period: String,
    pub fiscal_year: u32,
    pub entity_id: String,
    pub entity_name: String,
    pub cik: Option<String>,
    pub taxonomy: String,
    pub report_type: Option<String>,
    pub filing_url: Option<String>,
}

/// Filing data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Filing {
    pub accession_number: String,
    pub filing_date: String,
    pub accepted_date: Option<String>,
    pub form_type: String,
    pub entity_id: String,
    pub entity_name: String,
    pub cik: Option<String>,
    pub filing_url: Option<String>,
    pub reports: Vec<Report>,
}

/// Entity data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub cik: Option<String>,
    pub sic: Option<String>,
    pub ticker: Option<String>,
    pub lei: Option<String>,
    pub industry: Option<String>,
    pub sector: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}