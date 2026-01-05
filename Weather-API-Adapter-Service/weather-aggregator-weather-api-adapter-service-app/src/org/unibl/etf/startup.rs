use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_validator::QueryConfig;
use chrono::Utc;
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use tracing_actix_web::TracingLogger;
use crate::org::unibl::etf::configuration::settings::{Settings};
use crate::org::unibl::etf::controllers::current_weather_controller;
use crate::org::unibl::etf::handlers::query_error_handler;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "Weather Aggregator Weather API Adapter Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}

pub fn run(
    tcp_listener: TcpListener,
    settings: Settings,
    redis_pool: deadpool_redis::Pool
) -> std::io::Result<Server> {
    let http_client = web::Data::new(
        ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .build()
    );
    let current_weather_service =
        web::Data::new(CurrentWeatherService::default());
    let redis_pool = web::Data::new(redis_pool);
    let settings =
        web::Data::new(settings);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(http_client.clone())
            .app_data(current_weather_service.clone())
            .app_data(settings.clone())
            .wrap(TracingLogger::default())
            .app_data(redis_pool.clone())
            .app_data(QueryConfig::default()
                .error_handler(query_error_handler::handle_validation_error)
            )
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