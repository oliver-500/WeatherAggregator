use std::collections::HashMap;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::StatusCode;
use chrono::Utc;
use crate::org::unibl::etf::configuration::settings::ProviderSettings;
use crate::org::unibl::etf::controllers::current_weather_controller;
use crate::org::unibl::etf::middlewares::error_handler_middleware::internal_error_handler;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;
use crate::org::unibl::etf::services::openweather_api_service::OpenWeatherAPIService;
use crate::org::unibl::etf::services::weather_api_service::WeatherAPIService;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "External API Adapter Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}

pub fn run(
    tcp_listener: TcpListener,
    providers: HashMap<String, ProviderSettings>
) -> std::io::Result<Server> {

    let http_client =
        web::Data::new(reqwest::Client::new());
    let openweather_api_service =
        web::Data::new(OpenWeatherAPIService::default());
    let weather_api_service =
        web::Data::new(WeatherAPIService::default());
    let providers =
        web::Data::new(providers);
    let geocoding_service =
        web::Data::new(GeocodingService::default());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(openweather_api_service.clone())
            .app_data(weather_api_service.clone())
            .app_data(http_client.clone())
            .app_data(providers.clone())
            .app_data(geocoding_service.clone())
            .service(
                web::scope("/api/v1")
                    .configure(current_weather_controller::routes)
            )
            .route("/health_check", web::get().to(health_check))
            .wrap(middleware::ErrorHandlers::new()
                .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error_handler)
                .handler(StatusCode::BAD_REQUEST, internal_error_handler)
                .handler(StatusCode::NOT_FOUND, internal_error_handler)
            )

        })
        .listen(tcp_listener)?
        .run();

    Ok(server)

}