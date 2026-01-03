
use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_opentelemetry::RequestTracing;
use actix_web_validator::QueryConfig;
use chrono::Utc;

use reqwest_middleware::{ClientBuilder};
use reqwest_tracing::TracingMiddleware;
use crate::org::unibl::etf::configuration::Settings;
use crate::org::unibl::etf::controllers::current_weather_controller;
use crate::org::unibl::etf::model::errors::query_error_handler;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "External API Adapter Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}

use async_trait::async_trait;
use http::Extensions;
use reqwest::Request;
use reqwest_middleware::{Middleware, Next, Result};

struct LogHeaders;

#[async_trait]
impl Middleware for LogHeaders {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<reqwest::Response> {
        println!("Outgoing request:");
        println!("  {} {}", req.method(), req.url());

        for (k, v) in req.headers() {
            println!("  {}: {:?}", k, v);
        }

        next.run(req, extensions).await
    }
}

pub fn run(
    tcp_listener: TcpListener,
    configuration: Settings,
) -> std::io::Result<Server> {
    let http_client = web::Data::new(
        ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .with(LogHeaders)
            .build()
    );
    let current_weather_service = web::Data::new(CurrentWeatherService::default());
    let providers_settings = web::Data::new(configuration.providers);
    let cache_service_settings = web::Data::new(configuration.cache_service);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(http_client.clone())
            .app_data(current_weather_service.clone())
            .app_data(providers_settings.clone())
            .app_data(cache_service_settings.clone())
            .app_data(QueryConfig::default().error_handler(query_error_handler::handle_validation_error))
            .wrap(RequestTracing::new())
            .service(
                web::scope("/api/v1")
                    .configure(current_weather_controller::routes)
            )
            .route("/health_check", web::get().to(health_check))
    })
        .listen(tcp_listener)?
        .run();

    Ok(server)

}