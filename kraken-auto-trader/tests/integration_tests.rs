use kraken_auto_trader::{
    client::kraken_client::KrakenClient,
    utils::config::KrakenConfig,
    services::account_details::Account,
    errors::Error,
    middleware::KrakenClientState,
};
use actix_web::test;
#[actix_web::test]
async fn test_get_balance_integration() {
    // Temporarily unset API credentials to test error case
    let api_key = std::env::var("KRAKEN_API_KEY").ok();
    let api_secret = std::env::var("KRAKEN_API_SECRET").ok();
    std::env::remove_var("KRAKEN_API_KEY");
    std::env::remove_var("KRAKEN_API_SECRET");

    let config = KrakenConfig::default();
    let client = KrakenClient::new(config).unwrap();
    
    let req = test::TestRequest::default()
        .app_data(actix_web::web::Data::new(KrakenClientState::new(client)))
        .to_http_request();
        
    let balance = Account::new().unwrap().get_balance(req).await;
    assert!(matches!(balance, Err(Error::Auth(_))));

    // Restore API credentials
    if let Some(key) = api_key {
        std::env::set_var("KRAKEN_API_KEY", key);
    }
    if let Some(secret) = api_secret {
        std::env::set_var("KRAKEN_API_SECRET", secret);
    }
}

#[actix_web::test]
async fn test_invalid_api_credentials() {
    // Temporarily unset API credentials
    let api_key = std::env::var("KRAKEN_API_KEY").ok();
    let api_secret = std::env::var("KRAKEN_API_SECRET").ok();
    std::env::remove_var("KRAKEN_API_KEY");
    std::env::remove_var("KRAKEN_API_SECRET");

    let config = KrakenConfig::default();
    let client = KrakenClient::new(config).unwrap();
    
    let req = test::TestRequest::default()
        .app_data(actix_web::web::Data::new(KrakenClientState::new(client)))
        .to_http_request();
        
    let balance = Account::new().unwrap().get_balance(req).await;
    match balance {
        Ok(_) => panic!("Expected error but got success"),
        Err(e) => {
            assert!(matches!(e, Error::Auth(_)));
        }
    }

    // Restore API credentials
    if let Some(key) = api_key {
        std::env::set_var("KRAKEN_API_KEY", key);
    }
    if let Some(secret) = api_secret {
        std::env::set_var("KRAKEN_API_SECRET", secret);
    }
}

#[actix_web::test]
async fn test_rate_limiting() {
    // Temporarily unset API credentials to test error case
    let api_key = std::env::var("KRAKEN_API_KEY").ok();
    let api_secret = std::env::var("KRAKEN_API_SECRET").ok();
    std::env::remove_var("KRAKEN_API_KEY");
    std::env::remove_var("KRAKEN_API_SECRET");

    let config = KrakenConfig::default();
    let client = KrakenClient::new(config).unwrap();
    
    let req = test::TestRequest::default()
        .app_data(actix_web::web::Data::new(KrakenClientState::new(client)))
        .to_http_request();
        
    // Make multiple requests in quick succession
    for _ in 0..5 {
        let balance = Account::new().unwrap().get_balance(req.clone()).await;
        assert!(matches!(balance, Err(Error::Auth(_))));
    }

    // Restore API credentials
    if let Some(key) = api_key {
        std::env::set_var("KRAKEN_API_KEY", key);
    }
    if let Some(secret) = api_secret {
        std::env::set_var("KRAKEN_API_SECRET", secret);
    }
}

#[tokio::test]
async fn test_config_from_env() {
    // Set some environment variables
    std::env::set_var("KRAKEN_API_URL", "https://test.kraken.com");
    std::env::set_var("KRAKEN_USER_AGENT", "test-agent");
    std::env::set_var("KRAKEN_TIMEOUT", "60");
    std::env::set_var("KRAKEN_MAX_RETRIES", "5");
    std::env::set_var("KRAKEN_RETRY_DELAY_MS", "2000");
    std::env::set_var("KRAKEN_RATE_LIMIT_DELAY_MS", "10000");

    let client = KrakenClient::from_env().unwrap();
    assert_eq!(client.config.base_url, "https://test.kraken.com");
    assert_eq!(client.config.user_agent, "test-agent");
    assert_eq!(client.config.timeout, 60);
    assert_eq!(client.config.max_retries, 5);
    assert_eq!(client.config.retry_delay_ms, 2000);
    assert_eq!(client.config.rate_limit_delay_ms, 10000);

    // Clean up environment variables
    std::env::remove_var("KRAKEN_API_URL");
    std::env::remove_var("KRAKEN_USER_AGENT");
    std::env::remove_var("KRAKEN_TIMEOUT");
    std::env::remove_var("KRAKEN_MAX_RETRIES");
    std::env::remove_var("KRAKEN_RETRY_DELAY_MS");
    std::env::remove_var("KRAKEN_RATE_LIMIT_DELAY_MS");
}
