mod mocks;

use mocks::server::MockXbrlServer;
use rstest::rstest;
use tokio::runtime::Runtime;
use xbrl_api_client::{
    api::{
        client::XbrlClient,
        models::SearchParams,
    },
    data::{
        facts::FactValue,
        taxonomy::Taxonomy,
    },
    utils::errors::XbrlApiError,
};

// Helper function to run async tests (replacing the macro)
fn run_async<F: std::future::Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}
 
// Create a helper struct to bundle the server and client together
struct TestContext {
    mock_server: MockXbrlServer,
    client: XbrlClient,
}

// Single fixture that creates both mock server and client
#[rstest::fixture]
fn test_context() -> TestContext {
    let mock_server = run_async(MockXbrlServer::start());
    
    // Create client with the mock server's URL
    let api_key = "test_api_key";
    let mut client = XbrlClient::new(api_key);
    client.set_base_url(&mock_server.url());
    
    TestContext {
        mock_server,
        client,
    }
}

#[rstest]
fn test_get_taxonomies(mut test_context: TestContext) {
    // Arrange - Configure mock using our bundled mock server
    println!("Mock server URL: {}", test_context.mock_server.url());
    run_async(test_context.mock_server.mock_taxonomies());
    
    // Act & Assert - Use the client from our bundled context
    run_async(async {
        println!("Requesting URL: {}/api/v1/taxonomies", test_context.client.base_url());
        
        let result = test_context.client.get_taxonomies().await;
        
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let taxonomies = result.unwrap();
        assert_eq!(taxonomies.len(), 3);
        assert_eq!(taxonomies[0].name, "us-gaap");
        assert_eq!(taxonomies[1].name, "ifrs");
        assert_eq!(taxonomies[2].name, "dei");
    });
}

#[rstest]
fn test_get_reports(mut test_context: TestContext) {
    // Arrange - use our bundled mock server
    run_async(test_context.mock_server.mock_reports("us-gaap"));
    
    // Act & Assert - use the client from our bundled context
    run_async(async {
        let result = test_context.client.get_reports("us-gaap").await;
        
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let reports = result.unwrap();
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0].id, "rpt-123456");
        assert_eq!(reports[0].entity_name, "Example Corp");
        assert_eq!(reports[1].id, "rpt-123457");
        assert_eq!(reports[1].fiscal_period, "Q1");
    });
}

#[rstest]
fn test_get_facts(mut test_context: TestContext) {
    // Arrange
    run_async(test_context.mock_server.mock_facts("rpt-123456"));
    
    // Act & Assert
    run_async(async {
        let result = test_context.client.get_facts("rpt-123456").await;
        
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let facts = result.unwrap();
        assert_eq!(facts.len(), 3);
        assert_eq!(facts[0].id, "fact-12345");
        assert_eq!(facts[0].concept_name, "Assets");
        
        // Check if we can match enum values correctly
        match &facts[0].value {
            FactValue::Number(val) => {
                assert_eq!(*val, 1000000.0);
            },
            _ => panic!("Expected a number value"),
        }
    });
}

#[rstest]
fn test_get_concept_details(mut test_context: TestContext) {
    // Arrange
    run_async(test_context.mock_server.mock_concept_details("us-gaap", "Assets"));
    
    // Act & Assert
    run_async(async {
        let result = test_context.client.get_concept_details("us-gaap", "Assets").await;
        
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let concept = result.unwrap();
        assert_eq!(concept.name, "Assets");
        assert_eq!(concept.taxonomy, "us-gaap");
        assert_eq!(concept.balance, Some("debit".to_string()));
        
        // Check references
        let references = concept.references.unwrap();
        assert_eq!(references.len(), 4);
        assert_eq!(references[0].name, "Publisher");
        assert_eq!(references[0].value, "FASB");
    });
}

#[rstest]
fn test_search(mut test_context: TestContext) {
    // Arrange
    run_async(test_context.mock_server.mock_search());
    
    // Act & Assert
    run_async(async {
        let search_params = SearchParams {
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
        
        let result = test_context.client.search(search_params).await;
        
        if let Err(ref e) = result {
            println!("Error: {:?}", e);
        }
        
        assert!(result.is_ok());
        let facts = result.unwrap();
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].concept_name, "Assets");
        assert_eq!(facts[1].concept_name, "Assets");
        assert_eq!(facts[1].entity_name, Some("Another Company".to_string()));
    });
}

#[rstest]
fn test_unauthorized_error(mut test_context: TestContext) {
    // Arrange
    run_async(test_context.mock_server.mock_unauthorized());
    
    // Act & Assert
    run_async(async {
        // Create request - no /api/v1/ prefix needed as we've removed it from the mock
        let request = reqwest::Client::new().get(format!("{}/unauthorized", test_context.mock_server.url()));
        
        let result = test_context.client.execute_request::<Vec<Taxonomy>>(request).await;
        
        assert!(result.is_err());
        match result {
            Err(XbrlApiError::ApiError { status_code, .. }) => {
                assert_eq!(status_code, 401);
            },
            _ => panic!("Expected an API error with status code 401"),
        }
    });
}

#[rstest]
fn test_server_error(mut test_context: TestContext) {
    // Arrange
    run_async(test_context.mock_server.mock_server_error());
    
    // Act & Assert
    run_async(async {
        // Create request - no /api/v1/ prefix needed
        let request = reqwest::Client::new().get(format!("{}/error", test_context.mock_server.url()));
        
        let result = test_context.client.execute_request::<Vec<Taxonomy>>(request).await;
        
        assert!(result.is_err());
        match result {
            Err(XbrlApiError::ApiError { status_code, .. }) => {
                assert_eq!(status_code, 500);
            },
            _ => panic!("Expected an API error with status code 500"),
        }
    });
}
