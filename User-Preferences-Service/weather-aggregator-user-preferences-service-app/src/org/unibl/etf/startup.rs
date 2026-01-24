
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web_validator::QueryConfig;
use chrono::Utc;
use jsonwebtoken::DecodingKey;
use rustls::ServerConfig;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::org::unibl::etf::configuration::Settings;
use crate::org::unibl::etf::controllers::user_preferences_controller;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::ChannelPool;
use crate::org::unibl::etf::handlers::query_error_handler;
use crate::org::unibl::etf::middlewares::conditional_blocker_middleware::ConditionalBlocker;
use crate::org::unibl::etf::middlewares::json_500_middleware::Json500Middleware;
use crate::org::unibl::etf::middlewares::jwt_middleware::JwtMiddleware;
use crate::org::unibl::etf::model::responses::health_check_response::HealthCheckResponse;
use crate::org::unibl::etf::publishers::user_publisher::UserPublisher;
use crate::org::unibl::etf::repositories::user_preferences_repository::UserPreferencesRepository;
use crate::org::unibl::etf::services::jwt_service::JwtService;
use crate::org::unibl::etf::services::user_preferences_service::UserPreferencesService;

async fn health_check() -> impl Responder {
    let res = HealthCheckResponse {
        status: "UP".to_string(),
        service_name: "User Preferences Service".to_string(),
        timestamp: Utc::now()
    };
    HttpResponse::Ok().json(res)

}


pub fn run(
    tcp_listener: TcpListener,
    configuration: Settings,
    server_config: Option<ServerConfig>,
    signer_jwt_public_key: DecodingKey,
    is_broker_up: Arc<AtomicBool>,
    is_db_up: Arc<AtomicBool>,
    db_pool: PgPool,
    broker_pool: Arc<ChannelPool>
) -> std::io::Result<Server> {
    let jwt_service = Arc::new(JwtService::new(
        signer_jwt_public_key,
        configuration.jwt
    ));

    let _user_publisher = UserPublisher {
        broker_pool
    };

    let user_preferences_repository = UserPreferencesRepository::new_with_db_pool(db_pool);

    let user_preferences_service = web::Data::new(
        UserPreferencesService {
            user_preferences_repository,
        }
    );

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(
                QueryConfig::default().error_handler(query_error_handler::handle_validation_error)
            )
            .app_data(
                actix_web_validator::JsonConfig::default().error_handler(query_error_handler::handle_validation_error)
            )
            .wrap(TracingLogger::default())
            .wrap(JwtMiddleware {
                jwt_service: Arc::clone(&jwt_service)
            })
            .wrap(Json500Middleware)
            .wrap(ConditionalBlocker::new(is_db_up.clone(), is_broker_up.clone()))
            .app_data(user_preferences_service.clone())
            .service(
                web::scope("/api/v1")
                    .configure(user_preferences_controller::routes)
            )
            .route("/health_check", web::get().to(health_check))
    });
    server = match server_config {
        Some(config) => server.listen_rustls_0_23(tcp_listener, config)?,
        None => server.listen(tcp_listener)?,
    };


    Ok(server.run())

}