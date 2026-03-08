use std::net::{IpAddr};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::dev::ConnectionInfo;
use actix_web_validator::Query;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::requests::current_weather_coordinates_query::CurrentWeatherCoordinatesQuery;
use crate::org::unibl::etf::model::requests::current_weather_ip_address_query::CurrentWeatherIpAddressQuery;
use crate::org::unibl::etf::model::requests::current_weather_location_query::CurrentWeatherLocationQuery;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request_by_coordinates::UpstreamCurrentWeatherRequestByCoordinates;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request_by_location::UpstreamCurrentWeatherRequestByLocation;
use crate::org::unibl::etf::services::current_weather_cache_service::CurrentWeatherCacheService;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;
use crate::org::unibl::etf::util::is_local_ip;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/current_weather_by_coordinates", web::get().to(get_current_weather_data_by_coordinates))
        .route("/current_weather_by_location", web::get().to(get_current_weather_data_by_location))
        .route("/current_weather_by_ip_address", web::get().to(get_current_weather_data_by_ip_address));
}

#[tracing::instrument(
    name = "Get Current Weather Data Controller",
    skip(
        http_client,
        current_weather_service,
        cache_service_settings,
        providers_configuration
    )
)]
async fn get_current_weather_data_by_coordinates(
    query: Query<UpstreamCurrentWeatherRequestByCoordinates>,
    http_client: web::Data<ClientWithMiddleware>,
    providers_configuration: web::Data<Vec<ProviderSettings>>,
    cache_service_settings: web::Data<CacheServiceSettings>,
    current_weather_service: web::Data<CurrentWeatherService>
) -> Result<impl Responder, GenericServiceError> {
    let data = CurrentWeatherCoordinatesQuery {
        request: query.into_inner(),
        cache_service: CurrentWeatherCacheService::default(),
    };

    let res = current_weather_service
        .get_current_weather(
            data,
            http_client,
            providers_configuration,
            cache_service_settings,
        )
        .await
        .and_then(|current_weather_data|
            Ok(HttpResponse::Ok()
                .json(current_weather_data)
            )
        )?;

    Ok(res)
}

#[tracing::instrument(
    name = "Get Current Weather Data Controller",
    skip(
        http_client,
        current_weather_service,
        cache_service_settings,
        providers_configuration
    )
)]
async fn get_current_weather_data_by_location(
    query: Query<UpstreamCurrentWeatherRequestByLocation>,
    http_client: web::Data<ClientWithMiddleware>,
    providers_configuration: web::Data<Vec<ProviderSettings>>,
    cache_service_settings: web::Data<CacheServiceSettings>,
    current_weather_service: web::Data<CurrentWeatherService>
) -> Result<impl Responder, GenericServiceError> {
    let query = CurrentWeatherLocationQuery {
        request: query.into_inner(),
        cache_service: CurrentWeatherCacheService::default()
    };
    
    let res = current_weather_service
        .get_current_weather(
            query,
            http_client,
            providers_configuration,
            cache_service_settings,
        )
        .await
        .and_then(|current_weather_data|
            Ok(HttpResponse::Ok()
                .json(current_weather_data)
            )
        )?;

    Ok(res)
}

#[tracing::instrument(
    name = "Get Current Weather Data Controller",
    skip(
        http_client,
        current_weather_service,
        cache_service_settings,
        providers_configuration)
)]
async fn get_current_weather_data_by_ip_address(
    http_client: web::Data<ClientWithMiddleware>,
    providers_configuration: web::Data<Vec<ProviderSettings>>,
    cache_service_settings: web::Data<CacheServiceSettings>,
    current_weather_service: web::Data<CurrentWeatherService>,
    conn: ConnectionInfo,
    req: HttpRequest
) -> Result<impl Responder, GenericServiceError> {
    let ip_str = conn.realip_remote_addr().unwrap_or("");

    let ip: IpAddr = ip_str.parse::<IpAddr>().map_err(|e| {
        tracing::error!("Error while parsing ip address: {:?}", e);
        GenericServiceError {
            error: GenericServiceErrorDetails::new_aggregator_error(AggregatorError::ServerError(None))
        }
    })?;

    let ip: IpAddr = if is_local_ip(ip) {
        let ip = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.parse::<IpAddr>().ok())
            .ok_or_else(|| GenericServiceError {
                error: GenericServiceErrorDetails::new_aggregator_error(
                    AggregatorError::LocalIpError,
                ),
            })?;
        ip
    } else {
        ip
    };

    let query = CurrentWeatherIpAddressQuery {
        request: ip,
    };

    let res = current_weather_service
        .get_current_weather(
            query,
            http_client,
            providers_configuration,
            cache_service_settings,
        ).await
        .and_then(|current_weather_data|
            Ok(HttpResponse::Ok()
                .json(current_weather_data))
        )?;

    Ok(res)
}

