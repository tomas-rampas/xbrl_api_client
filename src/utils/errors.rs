use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum XbrlApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("API error: {status_code} - {message}")]
    ApiError {
        status_code: u16,
        message: String,
    },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("URL error: {0}")]
    UrlError(#[from] url::ParseError),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type XbrlResult<T> = Result<T, XbrlApiError>;