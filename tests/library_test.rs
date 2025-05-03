use xbrl_api_client::api::client::XbrlClient;
use xbrl_api_client::api::models::SearchParams;
use xbrl_api_client::data::facts::FactValue;

// This test is marked as ignored because it requires real API access
// Run with `cargo test -- --ignored` if you have a valid API key
#[tokio::test]
#[ignore]
async fn test_library_integration() {
    // Get API key from environment
    let api_key = std::env::var("XBRL_API_KEY").expect("XBRL_API_KEY must be set");
    let client = XbrlClient::new(&api_key);
    
    // Test taxonomies
    let taxonomies = client.get_taxonomies().await.expect("Failed to get taxonomies");
    assert!(!taxonomies.is_empty(), "Expected at least one taxonomy");
    println!("Found {} taxonomies", taxonomies.len());
    
    // Find a suitable taxonomy for testing
    let test_taxonomy = taxonomies
        .iter()
        .find(|t| t.name == "us-gaap" || t.name == "ifrs")
        .expect("No suitable test taxonomy found")
        .name
        .clone();
    
    println!("Using taxonomy: {}", test_taxonomy);
    
    // Test reports
    let reports = client
        .get_reports(&test_taxonomy)
        .await
        .expect("Failed to get reports");
    
    if reports.is_empty() {
        println!("No reports found for taxonomy {}", test_taxonomy);
        return;
    }
    
    println!("Found {} reports", reports.len());
    
    // Test facts for the first report
    let first_report = &reports[0];
    println!("Testing facts for report: {}", first_report.id);
    
    let facts = client
        .get_facts(&first_report.id)
        .await
        .expect("Failed to get facts");
    
    println!("Found {} facts", facts.len());
    
    // Print some facts to verify the data looks correct
    if !facts.is_empty() {
        println!("First few facts:");
        for (i, fact) in facts.iter().take(3).enumerate() {
            let value_str = match &fact.value {
                FactValue::String(s) => format!("String: {}", s),
                FactValue::Number(n) => format!("Number: {}", n),
                FactValue::Boolean(b) => format!("Boolean: {}", b),
            };
            println!(
                "{}. {} ({}): {}",
                i + 1,
                fact.concept_name,
                fact.period_end,
                value_str
            );
        }
    }
    
    // Test search functionality
    println!("Testing search functionality");
    let search_params = SearchParams {
        taxonomy: test_taxonomy,
        concept_name: Some("Assets".to_string()),
        entity_id: None,
        fiscal_year: None,  // No specific year for the test
        fiscal_period: None,
        dimension_name: None,
        member_name: None,
        text_search: None,
        value_greater_than: None,
        value_less_than: None,
    };
    
    let search_results = client
        .search(search_params)
        .await
        .expect("Failed to perform search");
    
    println!("Search found {} results", search_results.len());
    
    // The test passes if we get here without errors
    assert!(true);
}