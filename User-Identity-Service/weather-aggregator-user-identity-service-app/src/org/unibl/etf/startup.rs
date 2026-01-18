
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_validator::QueryConfig;
use chrono::Utc;
use rustls::ServerConfig;
use tracing_actix_web::TracingLogger;
use crate::org::unibl::etf::configuration::Settings;
use crate::org::unibl::etf::controllers::auth_controller;
use crate::org::unibl::etf::handlers::query_error_handler;
use crate::org::unibl::etf::middlewares::json_500_middleware::Json500Middleware;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::repositories::user_identity_repository::UserIdentityRepository;
use crate::org::unibl::etf::services::auth_service::AuthService;
use crate::org::unibl::etf::services::jwt_service::JwtService;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "User Identity Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}


pub fn run(
    tcp_listener: TcpListener,
    _configuration: Settings,
    server_config: Option<ServerConfig>,
    jwt_private_key: Vec<u8>,
    jwt_public_key: Vec<u8>,
    _is_broker_up: Arc<AtomicBool>,
    _is_db_up: Arc<AtomicBool>
) -> std::io::Result<Server> {
    let jwt_service = JwtService::new_with_private_key(jwt_private_key, jwt_public_key);
    let user_identity_repository = UserIdentityRepository::default();

    let auth_service = web::Data::new(
        AuthService {
            jwt_service,
            user_identity_repository
        }
    );

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .app_data(QueryConfig::default().error_handler(query_error_handler::handle_validation_error))
            .wrap(TracingLogger::default())
            .wrap(Json500Middleware)
            .service(
                web::scope("/api/v1")
                    .configure(auth_controller::routes)
            )
            .route("/health_check", web::get().to(health_check))
    });
    server = match server_config {
        Some(config) => server.listen_rustls_0_23(tcp_listener, config)?,
        None => server.listen(tcp_listener)?,
    };


    Ok(server.run())

}