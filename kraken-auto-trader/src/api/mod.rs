use actix_web::web;

pub mod handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::hello)
        .service(handlers::get_balance)
        .service(handlers::get_trade_volume)
        .service(handlers::get_system_status)
        .service(handlers::get_server_time)
        .service(handlers::get_ticker)
        .service(handlers::get_recent_trades)
        .service(handlers::get_recent_spreads);
    // .service(handlers::get_open_orders);
}
