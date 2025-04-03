use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Balance = Option<HashMap<String, String>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedBalance {
    pub balance: Option<HashMap<String, String>>,
    pub credit: Option<HashMap<String, String>>,
    pub credit_used: Option<HashMap<String, String>>,
    pub hold_trade: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeBalance {
    pub eb: String,        // Equivalent balance (combined balance of all currencies)
    pub tb: String,        // Trade balance (combined balance of all equity currencies)
    pub m: String,         // Margin amount of open positions
    pub n: String,         // Unrealized net profit/loss of open positions
    pub c: String,         // Cost basis of open positions
    pub v: String,         // Current floating valuation of open positions
    pub e: String,         // Equity = trade balance + unrealized net profit/loss
    pub mf: String,        // Free margin = equity - initial margin (maximum margin available to open new positions)
    pub ml: String,        // Margin level = (equity / initial margin) * 100
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDescription {
    pub pair: String,
    pub r#type: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub refid: String,
    pub userref: Option<String>,
    pub status: String,
    pub opentm: f64,
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub ordertxid: String,
    pub pair: String,
    pub time: f64,
    pub r#type: String,
    pub ordertype: String,
    pub price: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub margin: String,
    pub misc: String,
    pub ledgers: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub ordertxid: String,
    pub posstatus: String,
    pub pair: String,
    pub time: f64,
    pub r#type: String,
    pub ordertype: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub vol_closed: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub orders: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ledger {
    pub refid: String,
    pub time: f64,
    pub r#type: String,
    pub aclass: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub balance: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeTier {
    pub fee: String,
    pub minfee: String,
    pub maxfee: String,
    pub nextfee: String,
    pub nextvolume: String,
    pub tiervolume: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeVolume {
    pub currency: String,
    pub volume: String,
    pub fees: Option<HashMap<String, FeeTier>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportReport {
    pub id: String,
    pub status: String,
    pub created_at: f64,
    pub completed_at: Option<f64>,
    pub expires_at: Option<f64>,
    pub report_type: String,
    pub format: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrders {
    pub open: HashMap<String, Order>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedOrders {
    pub closed: HashMap<String, Order>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesHistory {
    pub trades: HashMap<String, Trade>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenPositions {
    pub positions: HashMap<String, Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledgers {
    pub ledger: HashMap<String, Ledger>,
    pub count: i64,
}