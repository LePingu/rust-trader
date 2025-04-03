use actix_web::{dev::ServiceRequest, Error, error::ErrorUnauthorized};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::client::kraken_client::KrakenClient;

#[derive(Clone)]
pub struct KrakenClientState {
    pub client: Arc<Mutex<KrakenClient>>,
}

impl KrakenClientState {
    pub fn new(client: KrakenClient) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn get_client(&self) -> KrakenClient {
        self.client.lock().await.clone()
    }
}

pub struct KrakenClientMiddleware;

impl KrakenClientMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S> actix_web::dev::Transform<S, ServiceRequest> for KrakenClientMiddleware
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
    > + 'static,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse;
    type Error = Error;
    type Transform = KrakenClientMiddlewareService<S>;
    type InitError = ();
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Transform, Self::InitError>> + 'static>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move { Ok(KrakenClientMiddlewareService { service }) })
    }
}

pub struct KrakenClientMiddlewareService<S> {
    service: S,
}

impl<S> actix_web::dev::Service<ServiceRequest> for KrakenClientMiddlewareService<S>
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
    >,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        self.service.call(req)
    }
}

// Helper trait to extract client from request
pub trait KrakenClientExt {
    fn get_client(&self) -> Result<Arc<Mutex<KrakenClient>>, Error>;
}

impl KrakenClientExt for actix_web::HttpRequest {
    fn get_client(&self) -> Result<Arc<Mutex<KrakenClient>>, Error> {
        self.app_data::<actix_web::web::Data<KrakenClientState>>()
            .map(|data| data.client.clone())
            .ok_or_else(|| ErrorUnauthorized("Kraken client not found"))
    }
} 