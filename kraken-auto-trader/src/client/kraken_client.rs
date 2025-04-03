use crate::{errors::Error, utils::config::KrakenConfig};

use super::rate_limit::RateLimiter;

use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{debug, error, info};

#[derive(Debug, Deserialize)]
pub struct KrakenResponse<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

#[derive(Debug, Clone)]
pub struct KrakenClient {
    pub config: KrakenConfig,
    pub(crate) client: Client,
    pub(crate) rate_limiter: RateLimiter,
}

/// Convert a HashMap to a URL encoded string
pub fn encode_params(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

impl KrakenClient {
    /// Create a new Kraken API client with the given configuration
    pub fn new(config: KrakenConfig) -> Result<Self, Error> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent(&config.user_agent)
            .build()?;

        // Create rate limiter with 15 requests per minute (Kraken's limit)
        let rate_limiter = RateLimiter::new(15, 0.25); // 15 tokens, 0.25 tokens per second (15/60)

        Ok(Self {
            config,
            client,
            rate_limiter,
        })
    }

    /// Create a new Kraken API client with default configuration
    pub fn default() -> Result<Self, Error> {
        Self::new(KrakenConfig::default())
    }

    /// Create a new Kraken API client with configuration from environment
    pub fn from_env() -> Result<Self, Error> {
        let config = KrakenConfig::from_env().map_err(|e| Error::Unknown(e.to_string()))?;
        Self::new(config)
    }

    /// Get the HTTP client
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// Make a GET request with retry logic
    pub async fn make_get_request_with_retry<T>(
        &mut self,
        url: String,
        endpoint: String,
    ) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut retries = 0;
        loop {
            match self.make_single_get_request(&url, &endpoint).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if !e.is_retryable() || retries >= self.config.max_retries {
                        return Err(e);
                    }
                    retries += 1;
                    let delay = self.config.retry_delay() * retries as u32;
                    debug!(
                        "Retrying GET request after {:?} delay (attempt {}/{})",
                        delay, retries, self.config.max_retries
                    );
                    sleep(delay).await;
                }
            }
        }
    }

    /// Make a single GET request without retry logic
    async fn make_single_get_request<T>(
        &mut self,
        url: &str,
        endpoint: &str,
    ) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Wait for rate limit token
        self.rate_limiter.acquire().await;

        info!("Making GET request to {}", endpoint);
        let request = self.client.get(url);

        let response = request
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(response.status().to_string()));
        }

        let kraken_response: KrakenResponse<T> = response.json().await?;

        if !kraken_response.error.is_empty() {
            error!("Kraken API error: {:?}", kraken_response.error);
            return Err(Error::from(kraken_response.error));
        }

        match kraken_response.result {
            Some(result) => Ok(result),
            None => Err(Error::Unknown("No data received from API".into())),
        }
    }

    /// Make a request with retry logic
    pub async fn make_request_with_retry<T>(
        &mut self,
        url: String,
        headers: HashMap<String, String>,
        post_data: String,
        endpoint: String,
    ) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut retries = 0;
        loop {
            match self
                .make_single_request(&url, headers.clone(), &post_data, &endpoint)
                .await
            {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if !e.is_retryable() || retries >= self.config.max_retries {
                        return Err(e);
                    }
                    retries += 1;
                    let delay = self.config.retry_delay() * retries as u32;
                    debug!(
                        "Retrying request after {:?} delay (attempt {}/{})",
                        delay, retries, self.config.max_retries
                    );
                    sleep(delay).await;
                }
            }
        }
    }

    /// Make a single request without retry logic
    async fn make_single_request<T>(
        &mut self,
        url: &str,
        headers: HashMap<String, String>,
        post_data: &str,
        endpoint: &str,
    ) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Wait for rate limit token
        self.rate_limiter.acquire().await;

        info!("Making request to {}", endpoint);
        let mut request = self.client.post(url);

        // Add all headers
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_data.to_string())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(response.status().to_string()));
        }

        let kraken_response: KrakenResponse<T> = response.json().await?;

        if !kraken_response.error.is_empty() {
            error!("Kraken API error: {:?}", kraken_response.error);
            return Err(Error::from(kraken_response.error));
        }

        match kraken_response.result {
            Some(result) => Ok(result),
            None => Err(Error::Unknown("No data received from API".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_encode_params() {
        let mut params = HashMap::new();
        params.insert("key1".to_string(), "value1".to_string());
        params.insert("key2".to_string(), "value2".to_string());

        let encoded = encode_params(&params);
        assert!(encoded.contains("key1=value1"));
        assert!(encoded.contains("key2=value2"));
    }

    #[test]
    fn test_kraken_client_creation() {
        let config = KrakenConfig::default();
        let client = KrakenClient::new(config);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.config.timeout, 30);
        assert_eq!(client.rate_limiter.capacity(), 15);
        assert_eq!(client.rate_limiter.rate(), 0.25);
    }

    #[test]
    fn test_kraken_client_default() {
        let client = KrakenClient::default();
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(client.config.user_agent.contains("kraken_client/"));
        assert_eq!(client.config.timeout, 30);
    }

    // #[tokio::test]
    // async fn test_private_request_with_invalid_credentials() {
    //     // Temporarily unset API credentials
    //     let api_key = std::env::var("KRAKEN_API_KEY").ok();
    //     let api_secret = std::env::var("KRAKEN_API_SECRET").ok();
    //     std::env::remove_var("KRAKEN_API_KEY");
    //     std::env::remove_var("KRAKEN_API_SECRET");

    //     let mut client = KrakenClient::default().unwrap();
    //     let params = HashMap::new();
    //     let result = client
    //         .private_request::<Value>("/0/private/Balance", params)
    //         .await;

    //     match result {
    //         Ok(_) => panic!("Expected error but got success"),
    //         Err(e) => assert!(matches!(e, Error::Auth(_))),
    //     }

    //     // Restore API credentials
    //     if let Some(key) = api_key {
    //         std::env::set_var("KRAKEN_API_KEY", key);
    //     }
    //     if let Some(secret) = api_secret {
    //         std::env::set_var("KRAKEN_API_SECRET", secret);
    //     }
    // }
}
