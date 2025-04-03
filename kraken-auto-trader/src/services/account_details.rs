use crate::{
    client::kraken_apis::{KrakenRequest, PrivateApi, PrivateApiBuilder},
    errors::Error,
    models::account::{
        Balance, TradeVolume, TradeBalance, OpenOrders, ClosedOrders, TradesHistory,
        OpenPositions, Ledgers, ExportReport, Order, Trade, Ledger
    },
};
use actix_web::HttpRequest;
use std::collections::HashMap;
use crate::utils::endpoints::account::*;

pub struct Account {
    private_api: PrivateApi,
}

impl Account {
    pub fn new() -> Result<Self, Error> {
        let api = PrivateApiBuilder::from_env()?
            .build()?;

        Ok(Self { private_api: api })
    }

    /// Get account balance
    pub async fn get_balance(&self, req: HttpRequest) -> Result<Balance, Error> {
        PrivateApi::kraken_request(&self.private_api, req, BALANCE, HashMap::new()).await
    }

    /// Get extended account balance
    pub async fn get_balance_ex(&self, req: HttpRequest) -> Result<Balance, Error> {
        PrivateApi::kraken_request(&self.private_api, req, BALANCE_EX, HashMap::new()).await
    }

    /// Get trade balance
    pub async fn get_trade_balance(&self, req: HttpRequest, asset: Option<String>) -> Result<TradeBalance, Error> {
        let mut params = HashMap::new();
        if let Some(asset) = asset {
            params.insert("asset".to_string(), asset);
        }
        PrivateApi::kraken_request(&self.private_api, req, TRADE_BALANCE, params).await
    }

    /// Get open orders
    pub async fn get_open_orders(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        userref: Option<String>,
        cl_ord_id: Option<String>,
    ) -> Result<OpenOrders, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref);
        }
        if let Some(cl_ord_id) = cl_ord_id {
            params.insert("cl_ord_id".to_string(), cl_ord_id);
        }
        PrivateApi::kraken_request(&self.private_api, req, OPEN_ORDERS, params).await
    }

    /// Get closed orders
    pub async fn get_closed_orders(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        userref: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        ofs: Option<i64>,
        closetime: Option<String>,
        consolidate_taker: Option<bool>,
        without_count: Option<bool>,
    ) -> Result<ClosedOrders, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref);
        }
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        if let Some(closetime) = closetime {
            params.insert("closetime".to_string(), closetime);
        }
        if let Some(consolidate_taker) = consolidate_taker {
            params.insert("consolidate_taker".to_string(), consolidate_taker.to_string());
        }
        if let Some(without_count) = without_count {
            params.insert("without_count".to_string(), without_count.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, CLOSED_ORDERS, params).await
    }

    /// Query orders info
    pub async fn query_orders(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        userref: Option<String>,
        txid: Vec<String>,
        consolidate_taker: Option<bool>,
    ) -> Result<HashMap<String, Order>, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref);
        }
        params.insert("txid".to_string(), txid.join(","));
        if let Some(consolidate_taker) = consolidate_taker {
            params.insert("consolidate_taker".to_string(), consolidate_taker.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, QUERY_ORDERS, params).await
    }

    /// Get order amends
    pub async fn get_order_amends(
        &self,
        req: HttpRequest,
        order_id: String,
    ) -> Result<HashMap<String, Vec<Order>>, Error> {
        let mut params = HashMap::new();
        params.insert("order_id".to_string(), order_id);
        PrivateApi::kraken_request(&self.private_api, req, ORDER_AMENDS, params).await
    }

    /// Get trades history
    pub async fn get_trades_history(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        type_param: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        ofs: Option<i64>,
        consolidate_taker: Option<bool>,
    ) -> Result<TradesHistory, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        if let Some(type_param) = type_param {
            params.insert("type".to_string(), type_param);
        }
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        if let Some(consolidate_taker) = consolidate_taker {
            params.insert("consolidate_taker".to_string(), consolidate_taker.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, TRADES_HISTORY, params).await
    }

    /// Query trades info
    pub async fn query_trades(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        txid: Vec<String>,
        consolidate_taker: Option<bool>,
    ) -> Result<HashMap<String, Trade>, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        params.insert("txid".to_string(), txid.join(","));
        if let Some(consolidate_taker) = consolidate_taker {
            params.insert("consolidate_taker".to_string(), consolidate_taker.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, QUERY_TRADES, params).await
    }

    /// Get open positions
    pub async fn get_open_positions(
        &self,
        req: HttpRequest,
        trades: Option<bool>,
        docalcs: Option<bool>,
    ) -> Result<OpenPositions, Error> {
        let mut params = HashMap::new();
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        if let Some(docalcs) = docalcs {
            params.insert("docalcs".to_string(), docalcs.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, OPEN_POSITIONS, params).await
    }

    /// Get ledgers
    pub async fn get_ledgers(
        &self,
        req: HttpRequest,
        asset: Option<String>,
        aclass: Option<String>,
        type_param: Option<String>,
        start: Option<i64>,
        end: Option<i64>,
        ofs: Option<i64>,
        consolidate_taker: Option<bool>,
    ) -> Result<Ledgers, Error> {
        let mut params = HashMap::new();
        if let Some(asset) = asset {
            params.insert("asset".to_string(), asset);
        }
        if let Some(aclass) = aclass {
            params.insert("aclass".to_string(), aclass);
        }
        if let Some(type_param) = type_param {
            params.insert("type".to_string(), type_param);
        }
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        if let Some(consolidate_taker) = consolidate_taker {
            params.insert("consolidate_taker".to_string(), consolidate_taker.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, LEDGERS, params).await
    }

    /// Query ledgers
    pub async fn query_ledgers(
        &self,
        req: HttpRequest,
        id: Vec<String>,
    ) -> Result<HashMap<String, Ledger>, Error> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.join(","));
        PrivateApi::kraken_request(&self.private_api, req, QUERY_LEDGERS, params).await
    }

    /// Get trade volume
    pub async fn get_trade_volume(
        &self,
        req: HttpRequest,
        pair: Option<String>,
    ) -> Result<TradeVolume, Error> {
        let mut params = HashMap::new();
        if let Some(pair) = pair {
            params.insert("pair".to_string(), pair);
        }
        PrivateApi::kraken_request(&self.private_api, req, TRADE_VOLUME, params).await
    }

    /// Request export report
    pub async fn request_export_report(
        &self,
        req: HttpRequest,
        report_type: String,
        description: String,
        format: Option<String>,
        starttm: Option<i64>,
        endtm: Option<i64>,
    ) -> Result<ExportReport, Error> {
        let mut params = HashMap::new();
        params.insert("report".to_string(), report_type);
        params.insert("description".to_string(), description);
        if let Some(format) = format {
            params.insert("format".to_string(), format);
        }
        if let Some(starttm) = starttm {
            params.insert("starttm".to_string(), starttm.to_string());
        }
        if let Some(endtm) = endtm {
            params.insert("endtm".to_string(), endtm.to_string());
        }
        PrivateApi::kraken_request(&self.private_api, req, REQUEST_EXPORT_REPORT, params).await
    }

    /// Get export report status
    pub async fn get_export_report_status(
        &self,
        req: HttpRequest,
        report_id: String,
    ) -> Result<ExportReport, Error> {
        let mut params = HashMap::new();
        params.insert("report".to_string(), report_id);
        PrivateApi::kraken_request(&self.private_api, req, GET_EXPORT_REPORT_STATUS, params).await
    }

    /// Retrieve export
    pub async fn retrieve_export(
        &self,
        req: HttpRequest,
        report_id: String,
    ) -> Result<ExportReport, Error> {
        let mut params = HashMap::new();
        params.insert("report".to_string(), report_id);
        PrivateApi::kraken_request(&self.private_api, req, RETRIEVE_EXPORT, params).await
    }

    /// Delete export report
    pub async fn delete_export_report(
        &self,
        req: HttpRequest,
        report_id: String,
    ) -> Result<(), Error> {
        let mut params = HashMap::new();
        params.insert("report".to_string(), report_id);
        PrivateApi::kraken_request(&self.private_api, req, DELETE_EXPORT_REPORT, params).await
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
    async fn test_get_balance() {
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

        let account = Account::new().expect("Failed to create Account");
        let balance = account.get_balance(req).await;
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
    async fn test_get_trade_volume() {
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

        let account = Account::new().expect("Failed to create Account");
        let volume = account.get_trade_volume(req, None).await;
        assert!(matches!(volume, Err(Error::Auth(_))));

        // Restore API credentials
        if let Some(key) = api_key {
            std::env::set_var("KRAKEN_API_KEY", key);
        }
        if let Some(secret) = api_secret {
            std::env::set_var("KRAKEN_API_SECRET", secret);
        }
    }
}
