use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl Error {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::RateLimitExceeded(_) | Error::NetworkError(_) | Error::TimeoutError(_)
        )
    }
}

// Implement conversion from Kraken API error responses
impl From<Vec<String>> for Error {
    fn from(errors: Vec<String>) -> Self {
        if errors.is_empty() {
            return Error::Unknown("Empty error response".into());
        }

        let error = errors[0].to_lowercase();
        if error.contains("rate limit") {
            Error::RateLimitExceeded(errors.join(", "))
        } else if error.contains("authentication") {
            Error::Auth(errors.join(", "))
        } else if error.contains("validation") {
            Error::ValidationError(errors.join(", "))
        } else {
            Error::Api(errors.join(", "))
        }
    }
}
