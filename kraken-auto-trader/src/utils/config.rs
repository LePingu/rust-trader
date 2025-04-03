use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct KrakenConfig {
    /// Base URL for the Kraken API
    pub base_url: String,
    
    /// User agent string for requests
    pub user_agent: String,
    
    /// Request timeout in seconds
    pub timeout: u64,
    
    /// Maximum number of retries for failed requests
    pub max_retries: u32,
    
    /// Initial retry delay in milliseconds
    pub retry_delay_ms: u64,
    
    /// Rate limit delay in milliseconds
    pub rate_limit_delay_ms: u64,
}

impl Default for KrakenConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.kraken.com".to_string(),
            user_agent: format!("kraken_client/{}", env!("CARGO_PKG_VERSION")),
            timeout: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            rate_limit_delay_ms: 5000,
        }
    }
}

impl KrakenConfig {
    /// Create a new configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut config = Self::default();
        
        // Override defaults with environment variables if set
        if let Ok(url) = std::env::var("KRAKEN_API_URL") {
            config.base_url = url;
        }
        if let Ok(agent) = std::env::var("KRAKEN_USER_AGENT") {
            config.user_agent = agent;
        }
        if let Ok(timeout) = std::env::var("KRAKEN_TIMEOUT") {
            config.timeout = timeout.parse().unwrap_or(30);
        }
        if let Ok(retries) = std::env::var("KRAKEN_MAX_RETRIES") {
            config.max_retries = retries.parse().unwrap_or(3);
        }
        if let Ok(delay) = std::env::var("KRAKEN_RETRY_DELAY_MS") {
            config.retry_delay_ms = delay.parse().unwrap_or(1000);
        }
        if let Ok(delay) = std::env::var("KRAKEN_RATE_LIMIT_DELAY_MS") {
            config.rate_limit_delay_ms = delay.parse().unwrap_or(5000);
        }

        Ok(config)
    }

    /// Get the rate limit delay as a Duration
    pub fn rate_limit_delay(&self) -> Duration {
        Duration::from_millis(self.rate_limit_delay_ms)
    }

    /// Get the retry delay as a Duration
    pub fn retry_delay(&self) -> Duration {
        Duration::from_millis(self.retry_delay_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = KrakenConfig::default();
        assert!(config.base_url.contains("api.kraken.com"));
        assert!(config.user_agent.contains("kraken_client/"));
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_ms, 1000);
        assert_eq!(config.rate_limit_delay_ms, 5000);
    }

    #[test]
    fn test_config_from_env() {
        // Set some environment variables
        std::env::set_var("KRAKEN_API_URL", "https://test.kraken.com");
        std::env::set_var("KRAKEN_USER_AGENT", "test-agent");
        std::env::set_var("KRAKEN_TIMEOUT", "60");
        std::env::set_var("KRAKEN_MAX_RETRIES", "5");
        std::env::set_var("KRAKEN_RETRY_DELAY_MS", "2000");
        std::env::set_var("KRAKEN_RATE_LIMIT_DELAY_MS", "10000");

        let config = KrakenConfig::from_env().unwrap();
        assert_eq!(config.base_url, "https://test.kraken.com");
        assert_eq!(config.user_agent, "test-agent");
        assert_eq!(config.timeout, 60);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_delay_ms, 2000);
        assert_eq!(config.rate_limit_delay_ms, 10000);

        // Clean up environment variables
        std::env::remove_var("KRAKEN_API_URL");
        std::env::remove_var("KRAKEN_USER_AGENT");
        std::env::remove_var("KRAKEN_TIMEOUT");
        std::env::remove_var("KRAKEN_MAX_RETRIES");
        std::env::remove_var("KRAKEN_RETRY_DELAY_MS");
        std::env::remove_var("KRAKEN_RATE_LIMIT_DELAY_MS");
    }
} 