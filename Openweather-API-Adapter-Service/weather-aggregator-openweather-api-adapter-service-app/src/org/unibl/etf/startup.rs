use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_validator::QueryConfig;
use chrono::Utc;
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use rustls::ServerConfig;
use tracing_actix_web::TracingLogger;
use crate::org::unibl::etf::configuration::settings::{HttpClientTlsIdentityBundle, Settings};
use crate::org::unibl::etf::controllers::current_weather_controller;
use crate::org::unibl::etf::handlers::query_error_handler;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "Weather Aggregator Openweather API Adapter Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}

pub fn run(
    tcp_listener: TcpListener,
    settings: Settings,
    redis_pool: deadpool_redis::Pool,
    server_config: Option<ServerConfig>,
    client_config: HttpClientTlsIdentityBundle
) -> std::io::Result<Server> {

    let mut client_builder = reqwest::Client::builder();
    client_builder = client_builder
        .identity(client_config.identity)
        .add_root_certificate(client_config.ca_certificate);
    
    let http_client = web::Data::new(
        ClientBuilder::new(client_builder.build().unwrap())
            .with(TracingMiddleware::default())
            .build()
    );

    let redis_pool = web::Data::new(redis_pool);
    let current_weather_service =
        web::Data::new(CurrentWeatherService::default());
    let settings =
        web::Data::new(settings);
    let geocoding_service =
        web::Data::new(GeocodingService::default());

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(http_client.clone())
            .app_data(geocoding_service.clone())
            .app_data(current_weather_service.clone())
            .app_data(settings.clone())
            .app_data(redis_pool.clone())
            .wrap(TracingLogger::default())
            .app_data(QueryConfig::default()
                .error_handler(query_error_handler::handle_validation_error)
            )
            .service(
                web::scope("/api/v1")
                    .configure(current_weather_controller::routes)
            )
            .route("/health_check", web::get().to(health_check))
    });

    server = match server_config {
        Some(config) => server.listen_rustls_0_23(tcp_listener, config)?,
        None => server.listen(tcp_listener)?,
    };


    Ok(server.run())
}