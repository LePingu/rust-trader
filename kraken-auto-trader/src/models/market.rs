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
    #[deprecated]
    pub lot: String,
    pub pair_decimals: i32,
    pub cost_decimals: i32,
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
    pub costmin: String,
    pub tick_size: String,
    pub status: String,
    pub long_position_limit: Option<i32>,
    pub short_position_limit: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub a: Vec<String>, // Ask [price, whole lot volume, lot volume]
    pub b: Vec<String>, // Bid [price, whole lot volume, lot volume]
    pub c: Vec<String>, // Last trade closed [price, lot volume]
    pub v: Vec<String>, // Volume [today, last 24 hours]
    pub p: Vec<String>, // Volume weighted average price [today, last 24 hours]
    pub t: Vec<i32>,    // Number of trades [today, last 24 hours]
    pub l: Vec<String>, // Low [today, last 24 hours]
    pub h: Vec<String>, // High [today, last 24 hours]
    pub o: String,      // Today's opening price
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OHLCData {
    pub time: i64,      // Unix timestamp
    pub open: String,   // Opening price
    pub high: String,   // High price
    pub low: String,    // Low price
    pub close: String,  // Closing price
    pub vwap: String,   // Volume weighted average price
    pub volume: String, // Volume
    pub count: i32,     // Number of trades
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OHLCResponse {
    pub last: i64, // ID to be used as since when polling for new data
    pub data: Vec<OHLCData>,
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Trade {
    pub price: String,
    pub volume: String,
    pub time: f64,
    pub buy_sell: String,
    pub market_limit: String,
    pub miscellaneous: String,
    pub trade_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spread {
    pub time: i64,
    pub bid: String,
    pub ask: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecentTradesResponse {
    pub last: String,
    #[serde(flatten)]
    pub trades: HashMap<String, Vec<Trade>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentSpreadsResponse {
    pub last: u64, // ID to be used as since when polling for new spread data
    pub spreads: HashMap<String, Vec<SpreadEntry>>, // Spread data for each asset pair
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpreadEntry {
    pub time: i64,   // Unix timestamp of the spread
    pub bid: String, // Bid price
    pub ask: String, // Ask price
}
