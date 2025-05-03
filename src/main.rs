mod api;
mod data;
mod utils;

use api::client::XbrlClient;
use api::models::SearchParams;
use data::facts::FactValue;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("XBRL_API_KEY").expect("XBRL_API_KEY must be set");
    
    // Create client
    let client = XbrlClient::new(&api_key);
    
    // Example: Get taxonomies
    let taxonomies = client.get_taxonomies().await?;
    println!("Fetched {} taxonomies", taxonomies.len());
    
    println!("\nAvailable taxonomies:");
    for taxonomy in &taxonomies {
        println!("- {} ({})", taxonomy.name, taxonomy.version);
    }
    
    // Example: Get reports from a specific taxonomy
    if let Some(first_taxonomy) = taxonomies.first() {
        println!("\nFetching reports for taxonomy: {}", first_taxonomy.name);
        let reports = client.get_reports(&first_taxonomy.name).await?;
        println!("Fetched {} reports for taxonomy {}", reports.len(), first_taxonomy.name);
        
        // Example: Get facts from a report
        if let Some(first_report) = reports.first() {
            println!("\nFetching facts for report: {}", first_report.id);
            let facts = client.get_facts(&first_report.id).await?;
            println!("Fetched {} facts", facts.len());
            
            // Print first 5 facts as examples
            println!("\nExample facts:");
            for (i, fact) in facts.iter().take(5).enumerate() {
                let value_display = match &fact.value {
                    FactValue::String(s) => format!("\"{}\"", s),
                    FactValue::Number(n) => format!("{}", n),
                    FactValue::Boolean(b) => format!("{}", b),
                };
                
                println!("{}. {} = {}", i + 1, fact.concept_name, value_display);
            }
            
            // Example: Search for specific facts
            println!("\nPerforming search for 'Assets' in {}", first_taxonomy.name);
            let search_params = SearchParams {
                taxonomy: first_taxonomy.name.clone(),
                concept_name: Some("Assets".to_string()),
                entity_id: None,
                fiscal_year: None,  // Don't restrict by year for the example
                fiscal_period: None,
                dimension_name: None,
                member_name: None,
                text_search: None,
                value_greater_than: Some(1_000_000.0),
                value_less_than: None,
            };
            
            let search_results = client.search(search_params).await?;
            println!("Search results: {} facts found", search_results.len());
            
            if !search_results.is_empty() {
                println!("\nExample search results:");
                for (i, fact) in search_results.iter().take(3).enumerate() {
                    let value_display = match &fact.value {
                        FactValue::String(s) => format!("\"{}\"", s),
                        FactValue::Number(n) => format!("{}", n),
                        FactValue::Boolean(b) => format!("{}", b),
                    };
                    
                    println!(
                        "{}. {} ({}) = {}",
                        i + 1,
                        fact.concept_name,
                        fact.entity_name.as_deref().unwrap_or("Unknown"),
                        value_display
                    );
                }
            }
        }
    }
    
    Ok(())
}