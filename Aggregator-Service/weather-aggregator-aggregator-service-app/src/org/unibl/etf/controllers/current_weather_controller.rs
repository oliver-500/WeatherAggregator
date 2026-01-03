
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Query;
use opentelemetry::trace::TraceContextExt;

use reqwest_middleware::ClientWithMiddleware;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/current_weather").route(web::get().to(get_current_weather_data)));
}

#[tracing::instrument(name = "Get Current Weather Data Controller",
    skip(http_client, current_weather_service, cache_service_settings, providers_configuration))]
async fn get_current_weather_data(
    query: Query<UpstreamCurrentWeatherRequest>,
    http_client: web::Data<ClientWithMiddleware>,
    providers_configuration: web::Data<Vec<ProviderSettings>>,
    cache_service_settings: web::Data<CacheServiceSettings>,
    current_weather_service: web::Data<CurrentWeatherService>
) -> Result<impl Responder, GenericServiceError> {
    Ok(current_weather_service.get_current_weather_data(
        query.as_ref(),
        http_client.as_ref(),
        providers_configuration.as_ref(),
        cache_service_settings.as_ref()
    ).await
           .and_then(|current_weather_data| Ok(HttpResponse::Ok().json(current_weather_data)))
           .map_err(|e| {
               tracing::error!("Was not able to get current weather data with error: {:?}", e);
               GenericServiceError {
                   error: GenericServiceErrorDetails::new_aggregator_error(e)
               }
           })?
    )
}