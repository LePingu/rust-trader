use crate::middleware::KrakenClientExt;
use actix_web::HttpRequest;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

use crate::errors::Error;
use crate::utils::crypto::{generate_nonce, get_signature};

use super::kraken_client::encode_params;
pub trait KrakenRequest {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    async fn kraken_request<T: for<'de> Deserialize<'de>>(
        &self,
        req: HttpRequest,
        endpoint: &str,
        params: HashMap<String, String>,
    ) -> Result<T, Error>;
}

#[derive(Default)]
pub struct PrivateApiBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl PrivateApiBuilder {
    pub fn with_api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    pub fn with_api_secret(mut self, secret: String) -> Self {
        self.api_secret = Some(secret);
        self
    }

    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            api_key: Some(
                env::var("KRAKEN_API_KEY")
                    .map_err(|e| Error::Auth(format!("Missing API key: {}", e)))?,
            ),
            api_secret: Some(
                env::var("KRAKEN_API_SECRET")
                    .map_err(|e| Error::Auth(format!("Missing API secret: {}", e)))?,
            ),
        })
    }

    pub fn build(self) -> Result<PrivateApi, Error> {
        Ok(PrivateApi {
            api_key: self
                .api_key
                .ok_or_else(|| Error::Auth("API key not set".to_string()))?,
            api_secret: self
                .api_secret
                .ok_or_else(|| Error::Auth("API secret not set".to_string()))?,
        })
    }
}

pub struct PrivateApi {
    api_secret: String,
    api_key: String,
}

impl KrakenRequest for PrivateApi {
    fn new() -> Result<Self, Error> {
        PrivateApiBuilder::from_env()?.build()
    }

    async fn kraken_request<T: for<'de> Deserialize<'de>>(
        &self,
        req: HttpRequest,
        endpoint: &str,
        mut params: HashMap<String, String>,
    ) -> Result<T, Error> {
        let client = req.get_client().map_err(|e| Error::Auth(e.to_string()))?;
        let mut client = client.lock().await;

        // Generate nonce
        let nonce = generate_nonce();
        params.insert("nonce".to_string(), nonce.to_string());

        // Create post data
        let post_data = encode_params(&params);

        // Sign the request
        let signature = get_signature(endpoint, nonce, &post_data, &self.api_secret)?;

        // Create the URL
        let url: String = format!("{}{}", client.config.base_url, endpoint);

        // Create headers for private request
        let mut headers = HashMap::new();
        headers.insert("API-Key".to_string(), self.api_key.clone());
        headers.insert("API-Sign".to_string(), signature.clone());

        // Make the request with retry logic
        let response: Value = client
            .make_request_with_retry(url, headers, post_data, endpoint.to_string())
            .await?;

        serde_json::from_value(response).map_err(|e| Error::SerializationError(e))
    }
}

impl PrivateApi {
    pub fn builder() -> PrivateApiBuilder {
        PrivateApiBuilder::default()
    }
}

/// Public API client for Kraken
pub struct PublicApi;

impl PublicApi {
    pub fn new() -> Self {
        Self
    }
}

impl KrakenRequest for PublicApi {
    fn new() -> Result<Self, Error> {
        Ok(PublicApi)
    }

    async fn kraken_request<T: for<'de> Deserialize<'de>>(
        &self,
        req: HttpRequest,
        endpoint: &str,
        params: HashMap<String, String>,
    ) -> Result<T, Error> {
        let client = req.get_client().map_err(|e| Error::Auth(e.to_string()))?;
        let mut client = client.lock().await;

        // Build the URL with query parameters
        let mut url = format!("{}{}", client.config.base_url, endpoint);

        // Add query parameters if any
        if !params.is_empty() {
            url.push('?');
            url.push_str(&encode_params(&params));
        }

        // Make the GET request
        let response: Value = client
            .make_get_request_with_retry(url, endpoint.to_string())
            .await?;

        serde_json::from_value(response).map_err(|e| Error::SerializationError(e))
    }
}
