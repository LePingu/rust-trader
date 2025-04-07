use actix_web::{get, HttpResponse, Responder};

use crate::services::{account_details::Account, market_data::MarketData};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/balance")]
pub async fn get_balance(req: actix_web::HttpRequest) -> impl Responder {
    let account = Account::new().unwrap();
    match account.get_balance(req).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/trade-volume")]
pub async fn get_trade_volume(req: actix_web::HttpRequest) -> impl Responder {
    let account = Account::new().unwrap();
    match account
        .get_trade_volume(req, Some("ETHUSD".to_string()))
        .await
    {
        Ok(volume) => HttpResponse::Ok().json(volume),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/system-status")]
pub async fn get_system_status(req: actix_web::HttpRequest) -> impl Responder {
    let market = MarketData::new();
    match market.get_system_status(req).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/server-time")]
pub async fn get_server_time(req: actix_web::HttpRequest) -> impl Responder {
    let market = MarketData::new();
    match market.get_server_time(req).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/ticker")]
pub async fn get_ticker(req: actix_web::HttpRequest) -> impl Responder {
    let market = MarketData::new();
    match market.get_ticker(req, "XBTUSD".to_string()).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/recent-trades")]
pub async fn get_recent_trades(req: actix_web::HttpRequest) -> impl Responder {
    let market = MarketData::new();
    match market
        .get_recent_trades(req, "ETHGBP".to_string(), Some(1616663618), Some(10))
        .await
    {
        Ok(trades) => HttpResponse::Ok().json(trades),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/recent-spreads")]
pub async fn get_recent_spreads(req: actix_web::HttpRequest) -> impl Responder {
    let market = MarketData::new();
    match market
        .get_recent_spreads(req, "ETHGBP".to_string(), None)
        .await
    {
        Ok(trades) => HttpResponse::Ok().json(trades),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
