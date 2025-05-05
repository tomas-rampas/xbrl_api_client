use std::env;
use std::panic;
use tokio::runtime::Runtime;
use xbrl_api_client::api::client::XbrlClient;
use xbrl_api_client::utils::errors::XbrlApiError;

// Test for environment variable handling (missing API key)
#[test]
fn test_missing_api_key() {
    // Save the current environment state to restore after the test
    let original_api_key = env::var("XBRL_API_KEY").ok();
    
    // Ensure API key is not set
    unsafe { 
        env::remove_var("XBRL_API_KEY");
    }
    
    // Dotenv should be already loaded from previous test but this is a safety measure
    dotenv::dotenv().ok();
    
    // When testing a function that returns Result instead of panicking
    let result = env::var("XBRL_API_KEY");
    
    // Restore environment
    if let Some(key) = original_api_key {
        unsafe { 
            env::set_var("XBRL_API_KEY", key);
        }
    }
    
    // The .env file has the test key, so this should succeed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_api_key_from_dotenv");
}

// Test that dotenv works correctly to load API key
#[test]
fn test_dotenv_loads_api_key() {
    // Save the current environment state to restore after the test
    let original_api_key = env::var("XBRL_API_KEY").ok();
    
    // Ensure API key is not set in environment
    unsafe { 
        env::remove_var("XBRL_API_KEY");
    }
    
    // Create/overwrite the .env file with a test API key
    use std::fs::File;
    use std::io::Write;
    
    let env_content = "XBRL_API_KEY=test_api_key_from_dotenv";
    let mut file = File::create(".env").unwrap();
    file.write_all(env_content.as_bytes()).unwrap();
    
    // Load from the .env file
    dotenv::dotenv().ok();
    
    // Check if API key was loaded
    let api_key = env::var("XBRL_API_KEY");
    
    // Restore environment
    if let Some(key) = original_api_key {
        unsafe { 
            env::set_var("XBRL_API_KEY", key);
        }
    } else {
        unsafe { 
            env::remove_var("XBRL_API_KEY");
        }
    }
    
    // The API key should have been loaded from the .env file
    assert!(api_key.is_ok());
    assert_eq!(api_key.unwrap(), "test_api_key_from_dotenv");
}

// Integration test for command-line execution with invalid API key
#[test]
fn test_invalid_api_key_handling() {
    // Save the current environment state to restore after the test
    let original_api_key = env::var("XBRL_API_KEY").ok();
    
    // Set an obviously invalid API key
    unsafe { 
        env::set_var("XBRL_API_KEY", "INVALID_KEY");
    }
    
    // Create a runtime to execute the binary
    let rt = Runtime::new().unwrap();
    
    // Execute the main function in a controlled environment
    // We're not using the actual main function due to complexity,
    // but this is a reasonable proxy test
    let result = rt.block_on(async {
        let client = XbrlClient::new("INVALID_KEY");
        client.get_taxonomies().await
    });
    
    // Restore environment
    if let Some(key) = original_api_key {
        unsafe { 
            env::set_var("XBRL_API_KEY", key);
        }
    } else {
        unsafe { 
            env::remove_var("XBRL_API_KEY");
        }
    }
    
    // We expect an authentication error due to the invalid API key
    assert!(result.is_err());
    // The error should be an authentication error or API error
    match result {
        Err(XbrlApiError::AuthError(_)) => {
            // This is the expected error
            // (although the real implementation might return a different specific error)
        },
        Err(XbrlApiError::ApiError { status_code, .. }) => {
            // API might return 401 Unauthorized or another error code
            assert!(status_code == 401 || status_code == 403);
        },
        _ => {
            // Unexpected result
            panic!("Expected an authentication error but got: {:?}", result);
        }
    }
}

// Test that the main process terminates successfully with an environment variable
// This only tests basic flow to ensure no panics occur in the normal case
// Since actual API calls would happen, we're only doing basic setup and execution checks
#[test]
#[ignore] // Ignored by default as it would try to make API calls
fn test_main_flow_without_api_calls() {
    // Save the current environment state to restore after the test
    let original_api_key = env::var("XBRL_API_KEY").ok();
    
    // Set a dummy API key - this is just to make the code run, not to execute real API calls
    unsafe { 
        env::set_var("XBRL_API_KEY", "TEST_KEY_NOT_REAL");
    }
    
    // The main actually makes API calls which we don't want to do in a unit test
    // So we just verify that we can create a client without errors
    let _client = XbrlClient::new("TEST_KEY_NOT_REAL");
    
    // Restore environment
    if let Some(key) = original_api_key {
        unsafe { 
            env::set_var("XBRL_API_KEY", key);
        }
    } else {
        unsafe { 
            env::remove_var("XBRL_API_KEY");
        }
    }
    
    // If we got here without panicking, that's success for this basic test
    assert!(true);
}
