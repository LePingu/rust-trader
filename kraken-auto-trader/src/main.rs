use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use kraken_auto_trader::{
    api, client::kraken_client::KrakenClient, middleware::{KrakenClientMiddleware, KrakenClientState}, utils::config::KrakenConfig
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = KrakenConfig::default();
    let client = KrakenClient::new(config).unwrap();
    let client_state = KrakenClientState::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client_state.clone()))
            .wrap(KrakenClientMiddleware::new())
            .service(web::scope("/api").configure(api::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
