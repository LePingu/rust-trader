use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderResponse {
    pub descr: OrderDescription,
    pub txid: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDescription {
    pub order: String,
    pub close: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelOrderResponse {
    pub count: i32,
    pub pending: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrdersResponse {
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrdersAfterXResponse {
    pub current_time: String,
    pub trigger_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebSocketToken {
    pub token: String,
    pub expires: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderStatus {
    pub refid: Option<String>,
    pub userref: Option<String>,
    pub status: String,
    pub opentm: Option<f64>,
    pub starttm: Option<f64>,
    pub expiretm: Option<f64>,
    pub descr: OrderDescription,
    pub vol: String,
    pub vol_exec: String,
    pub cost: String,
    pub fee: String,
    pub price: String,
    pub stopprice: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
    pub reason: Option<String>,
    pub closetm: Option<f64>,
    pub trades: Option<Vec<String>>,
} 