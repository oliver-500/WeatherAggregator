use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_validator::QueryConfig;
use chrono::Utc;
use rustls::ServerConfig;
use tracing_actix_web::TracingLogger;
use crate::org::unibl::etf::configuration::settings::{RedisStoreSettings};
use crate::org::unibl::etf::controllers::current_weather_controller;
use crate::org::unibl::etf::handlers::query_error_handler;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::services::cache_service::{CacheService};

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "Weather Aggregator Cache Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}

pub fn run(
    tcp_listener: TcpListener,
    settings: RedisStoreSettings,
    redis_pool: deadpool_redis::Pool,
    server_config: Option<ServerConfig>,
) -> std::io::Result<Server> {

    let cache_service =
        web::Data::new(CacheService::default());
    let configuration_settings =
        web::Data::new(settings);
    let redis_pool = web::Data::new(redis_pool);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(cache_service.clone())
            .app_data(configuration_settings.clone())
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