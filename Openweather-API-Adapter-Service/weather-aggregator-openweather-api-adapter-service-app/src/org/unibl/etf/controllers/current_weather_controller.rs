use actix_web::{web, HttpResponse, Responder};

use actix_web_validator::Query;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{Settings};
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};

use crate::org::unibl::etf::model::requests::current_weather_request::{CurrentWeatherRequest};


use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/current_weather").route(web::get().to(get_current_weather_data)));
}

#[tracing::instrument(name = "Get Current Weather Data Controller",
    skip(http_client, current_weather_service, geocoding_service, settings))]
async fn get_current_weather_data(
    current_weather_service: web::Data<CurrentWeatherService>,
    geocoding_service: web::Data<GeocodingService>,
    query: Query<CurrentWeatherRequest>,
    http_client: web::Data<ClientWithMiddleware>,
    settings: web::Data<Settings>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> Result<impl Responder, GenericServiceError> {
    Ok(current_weather_service
        .get_current_weather(
            query.into_inner(),
            http_client.get_ref(),
            &settings,
            redis_pool.get_ref(),
            geocoding_service.get_ref()
        )
        .await
        .and_then(|weather_data| {
            tracing::info!("Successfully got current weather data from external API: {:?}", weather_data);
            Ok(HttpResponse::Ok().json(weather_data))
        }
        )
        .map_err(|e| {
            tracing::error!("Was not able to get current weather data by coordinates with error: {:?}", e.get_message());
            GenericServiceError {
                error: GenericServiceErrorDetails::new_adapter_error(&settings.provider.name, e)
            }
        })?)

}