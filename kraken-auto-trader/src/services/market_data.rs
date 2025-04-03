use crate::{
    client::kraken_apis::{KrakenRequest, PublicApi}, errors::Error, models::market::*, utils::endpoints::market::*
};
use actix_web::HttpRequest;
use std::collections::HashMap;

pub struct MarketData {
    public_api: PublicApi,
}

impl MarketData {
    pub fn new() -> Self {
        Self {
            public_api: PublicApi::new(),
        }
    }

    /// Get server time
    pub async fn get_server_time(&self, req: HttpRequest) -> Result<ServerTime, Error> {
        PublicApi::kraken_request(&self.public_api, req, SERVER_TIME, HashMap::new()).await
    }

    /// Get system status
    pub async fn get_system_status(&self, req: HttpRequest) -> Result<SystemStatus, Error> {
        PublicApi::kraken_request(&self.public_api, req, SYSTEM_STATUS, HashMap::new()).await
    }

    /// Get asset info
    pub async fn get_asset_info(
        &self,
        req: HttpRequest,
        asset: Option<String>,
        aclass: Option<String>,
    ) -> Result<HashMap<String, Asset>, Error> {
        let mut params = HashMap::new();
        if let Some(asset) = asset {
            params.insert("asset".to_string(), asset);
        }
        if let Some(aclass) = aclass {
            params.insert("aclass".to_string(), aclass);
        }
        PublicApi::kraken_request(&self.public_api, req, ASSET_INFO, params).await
    }

    /// Get tradable asset pairs
    pub async fn get_tradable_asset_pairs(
        &self,
        req: HttpRequest,
        pair: Option<String>,
        info: Option<String>,
    ) -> Result<HashMap<String, AssetPair>, Error> {
        let mut params = HashMap::new();
        if let Some(pair) = pair {
            params.insert("pair".to_string(), pair);
        }
        if let Some(info) = info {
            params.insert("info".to_string(), info);
        }
        PublicApi::kraken_request(&self.public_api, req, TRADABLE_ASSET_PAIRS, params).await
    }

    /// Get ticker information
    pub async fn get_ticker(
        &self,
        req: HttpRequest,
        pair: String,
    ) -> Result<HashMap<String, Ticker>, Error> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair);
        PublicApi::kraken_request(&self.public_api, req, TICKER, params).await
    }

    /// Get OHLC data
    pub async fn get_ohlc(
        &self,
        req: HttpRequest,
        pair: String,
        interval: Option<u32>,
        since: Option<u64>,
    ) -> Result<OHLC, Error> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair);
        if let Some(interval) = interval {
            params.insert("interval".to_string(), interval.to_string());
        }
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        PublicApi::kraken_request(&self.public_api, req, OHLC, params).await
    }

    /// Get order book
    pub async fn get_order_book(
        &self,
        req: HttpRequest,
        pair: String,
        count: Option<u32>,
    ) -> Result<HashMap<String, OrderBook>, Error> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair);
        if let Some(count) = count {
            params.insert("count".to_string(), count.to_string());
        }
        PublicApi::kraken_request(&self.public_api, req, ORDER_BOOK, params).await
    }

    /// Get recent trades
    pub async fn get_recent_trades(
        &self,
        req: HttpRequest,
        pair: String,
        since: Option<u64>,
    ) -> Result<RecentTradesResponse, Error> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair);
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        PublicApi::kraken_request(&self.public_api, req, RECENT_TRADES, params).await
    }

    /// Get recent spreads
    pub async fn get_recent_spreads(
        &self,
        req: HttpRequest,
        pair: String,
        since: Option<u64>,
    ) -> Result<RecentSpreadsResponse, Error> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair);
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        PublicApi::kraken_request(&self.public_api, req, RECENT_SPREADS, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::kraken_client::KrakenClient, utils::config::KrakenConfig, middleware::KrakenClientState,
    };
    use actix_web::test;

    #[actix_web::test]
    async fn test_get_server_time() {
        let config = KrakenConfig::default();
        let client = KrakenClient::new(config).unwrap();

        let req = test::TestRequest::default()
            .app_data(actix_web::web::Data::new(KrakenClientState::new(client)))
            .to_http_request();

        let market = MarketData::new();
        let result = market.get_server_time(req).await;
        assert!(result.is_ok());
        
        let time = result.unwrap();
        assert!(time.unixtime > 0);
        assert!(!time.rfc1123.is_empty());
    }
} 