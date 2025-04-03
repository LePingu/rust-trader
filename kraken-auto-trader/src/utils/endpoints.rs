pub mod account {
    pub const BALANCE: &str = "/0/private/Balance";
    pub const BALANCE_EX: &str = "/0/private/BalanceEx";
    pub const TRADE_BALANCE: &str = "/0/private/TradeBalance";
    pub const OPEN_ORDERS: &str = "/0/private/OpenOrders";
    pub const CLOSED_ORDERS: &str = "/0/private/ClosedOrders";
    pub const QUERY_ORDERS: &str = "/0/private/QueryOrders";
    pub const ORDER_AMENDS: &str = "/0/private/OrderAmends";
    pub const TRADES_HISTORY: &str = "/0/private/TradesHistory";
    pub const QUERY_TRADES: &str = "/0/private/QueryTrades";
    pub const OPEN_POSITIONS: &str = "/0/private/OpenPositions";
    pub const LEDGERS: &str = "/0/private/Ledgers";
    pub const QUERY_LEDGERS: &str = "/0/private/QueryLedgers";
    pub const TRADE_VOLUME: &str = "/0/private/TradeVolume";
    pub const REQUEST_EXPORT_REPORT: &str = "/0/private/RequestExportReport";
    pub const GET_EXPORT_REPORT_STATUS: &str = "/0/private/GetExportReportStatus";
    pub const RETRIEVE_EXPORT: &str = "/0/private/RetrieveExport";
    pub const DELETE_EXPORT_REPORT: &str = "/0/private/DeleteExportReport";
}

pub mod market {
    pub const SERVER_TIME: &str = "/0/public/Time";
    pub const SYSTEM_STATUS: &str = "/0/public/SystemStatus";
    pub const ASSET_INFO: &str = "/0/public/Assets";
    pub const TRADABLE_ASSET_PAIRS: &str = "/0/public/AssetPairs";
    pub const TICKER: &str = "/0/public/Ticker";
    pub const OHLC: &str = "/0/public/OHLC";
    pub const ORDER_BOOK: &str = "/0/public/Depth";
    pub const RECENT_TRADES: &str = "/0/public/Trades";
    pub const RECENT_SPREADS: &str = "/0/public/Spread";
}

pub mod trading {
    pub const ADD_ORDER: &str = "/0/private/AddOrder";
    pub const ADD_ORDER_BATCH: &str = "/0/private/AddOrderBatch";
    pub const AMEND_ORDER: &str = "/0/private/AmendOrder";
    pub const EDIT_ORDER: &str = "/0/private/EditOrder";
    pub const CANCEL_ORDER: &str = "/0/private/CancelOrder";
    pub const CANCEL_ALL_ORDERS: &str = "/0/private/CancelAll";
    pub const CANCEL_ALL_ORDERS_AFTER_X: &str = "/0/private/CancelAllOrdersAfterX";
    pub const CANCEL_ORDER_BATCH: &str = "/0/private/CancelOrderBatch";
    pub const GET_WEBSOCKETS_TOKEN: &str = "/0/private/GetWebSocketsToken";
}

pub mod funding {
    pub const DEPOSIT_METHODS: &str = "/0/private/DepositMethods";
    pub const DEPOSIT_ADDRESSES: &str = "/0/private/DepositAddresses";
    pub const RECENT_DEPOSITS_STATUS: &str = "/0/private/DepositStatus";
    pub const WITHDRAWAL_METHODS: &str = "/0/private/WithdrawMethods";
    pub const WITHDRAWAL_ADDRESSES: &str = "/0/private/WithdrawAddresses";
    pub const WITHDRAWAL_INFO: &str = "/0/private/WithdrawInfo";
    pub const WITHDRAW_FUNDS: &str = "/0/private/Withdraw";
    pub const RECENT_WITHDRAWALS_STATUS: &str = "/0/private/WithdrawStatus";
    pub const WITHDRAWAL_CANCELLATION: &str = "/0/private/WithdrawCancel";
    pub const WALLET_TRANSFER: &str = "/0/private/WalletTransfer";
} 