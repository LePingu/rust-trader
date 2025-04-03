use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositMethod {
    pub method: String,
    pub limit: Option<String>,
    pub fee: String,
    pub address_setup_fee: Option<String>,
    pub gen_address: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositAddress {
    pub address: String,
    pub expiretm: Option<String>,
    pub new: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositStatus {
    pub method: String,
    pub aclass: String,
    pub asset: String,
    pub refid: String,
    pub txid: String,
    pub info: String,
    pub amount: String,
    pub fee: String,
    pub time: i64,
    pub status: String,
    pub status_prop: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalMethod {
    pub method: String,
    pub limit: Option<String>,
    pub fee: String,
    pub address_setup_fee: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalAddress {
    pub address: String,
    pub asset: String,
    pub method: String,
    pub key: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalInfo {
    pub method: String,
    pub limit: String,
    pub amount: String,
    pub fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalResponse {
    pub refid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalStatus {
    pub method: String,
    pub aclass: String,
    pub asset: String,
    pub refid: String,
    pub txid: Option<String>,
    pub info: String,
    pub amount: String,
    pub fee: String,
    pub time: i64,
    pub status: String,
    pub status_prop: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTransfer {
    pub refid: String,
} 