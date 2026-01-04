use actix_web::{web, HttpResponse, Responder};

use actix_web_validator::Query;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{Settings};
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};

use crate::org::unibl::etf::model::requests::current_weather_request::{CurrentWeatherRequest};
use crate::org::unibl::etf::model::responses::uniform_current_weather_response::UniformCurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/current_weather").route(web::get().to(get_current_weather_data)));
}

#[tracing::instrument(name = "Get Current Weather Data Controller",
    skip(http_client, current_weather_service, settings))]
async fn get_current_weather_data(
    current_weather_service: web::Data<CurrentWeatherService>,
    query: Query<CurrentWeatherRequest>,
    http_client: web::Data<ClientWithMiddleware>,
    settings: web::Data<Settings>,
) -> Result<impl Responder, GenericServiceError> {
    let res = current_weather_service
        .get_current_weather_by_coordinates_or_location_name(
            query.as_ref(),
            http_client.get_ref(),
            &settings.provider
        )
        .await
        .and_then(|res| {
            tracing::info!("Successfully got current weather data from external API: {:?}", res);
            Ok(res)
        })
        .map_err(|e| {
            tracing::error!("Was not able to get current weather data by coordinates with error: {:?}", e.get_message());
            GenericServiceError {
                error: GenericServiceErrorDetails::new_adapter_error(&settings.provider.name, e)
            }
        })?;

    Ok(UniformCurrentWeatherResponse::try_from(res)
        .and_then(|weather_data| Ok(HttpResponse::Ok().json(weather_data)))
        .map_err(|e| {
            tracing::error!("Was not able to get transform weather data to uniform format with error: {:?}", e.get_message());
            GenericServiceError {
                error: GenericServiceErrorDetails::new_adapter_error(&settings.provider.name, e)
            }
        })?
    )

}