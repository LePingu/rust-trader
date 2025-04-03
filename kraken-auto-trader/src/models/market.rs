use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerTime {
    pub unixtime: i64,
    pub rfc1123: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemStatus {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub aclass: String,
    pub altname: String,
    pub decimals: i32,
    pub display_decimals: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPair {
    pub altname: String,
    pub wsname: Option<String>,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub pair_decimals: i32,
    pub lot_decimals: i32,
    pub lot_multiplier: i32,
    pub leverage_buy: Vec<i32>,
    pub leverage_sell: Vec<i32>,
    pub fees: Vec<Vec<String>>,
    pub fees_maker: Option<Vec<Vec<String>>>,
    pub fee_volume_currency: String,
    pub margin_call: i32,
    pub margin_stop: i32,
    pub ordermin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub a: Vec<String>,    // Ask [price, whole lot volume, lot volume]
    pub b: Vec<String>,    // Bid [price, whole lot volume, lot volume]
    pub c: Vec<String>,    // Last trade closed [price, lot volume]
    pub v: Vec<String>,    // Volume [today, last 24 hours]
    pub p: Vec<String>,    // Volume weighted average price [today, last 24 hours]
    pub t: Vec<i32>,       // Number of trades [today, last 24 hours]
    pub l: Vec<String>,    // Low [today, last 24 hours]
    pub h: Vec<String>,    // High [today, last 24 hours]
    pub o: String,         // Today's opening price
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OHLC {
    pub time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub vwap: String,
    pub volume: String,
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookEntry {
    pub price: String,
    pub volume: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBook {
    pub asks: Vec<OrderBookEntry>,
    pub bids: Vec<OrderBookEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub price: String,
    pub volume: String,
    pub time: f64,
    pub side: String,
    pub order_type: String,
    pub misc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spread {
    pub time: i64,
    pub bid: String,
    pub ask: String,
}

#[derive(Debug, Deserialize)]
pub struct RecentTradesResponse {
    pub pair: HashMap<String, Vec<Vec<String>>>,
    pub last: String,
}

#[derive(Debug, Deserialize)]
pub struct RecentSpreadsResponse {
    pub pair: HashMap<String, Vec<Vec<String>>>,
    pub last: u64,
} 