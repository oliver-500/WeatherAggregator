use actix_web::{web, HttpResponse, Responder};

use actix_web_validator::Query;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{Settings};
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};

use crate::org::unibl::etf::model::requests::current_weather_request::{CurrentWeatherRequest};
use crate::org::unibl::etf::model::responses::uniform_current_weather_response::UniformCurrentWeatherResponse;
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
) -> Result<impl Responder, GenericServiceError> {
    let (lat, lon) = if query.lat == None || query.lon == None {
        match geocoding_service.geocode_location(
            query.location_name.clone().unwrap_or("".to_string()).as_str(),
            http_client.get_ref(),
            5,
            &settings.geocoding_service
        ).await {
            Ok((lat, lon)) => {
                tracing::info!("Successfully geocoded location. Result: latitude {}, longitude {}", lat, lon);
                (lat, lon)
            },
            Err(e) => return {
                tracing::error!("Could not geocode location with error: {:?}", e);
                Err(
                    GenericServiceError {
                        error: GenericServiceErrorDetails::new_adapter_error(settings.provider.name.as_str(), e),
                    }
                )
            },
        }
    }
    else {
        (query.lat.unwrap_or(0.0), query.lon.unwrap_or(0.0))
    };

    let res = current_weather_service
        .get_current_weather_by_coordinates(
            (lat, lon),
            http_client.get_ref(),
            &settings.provider
        )
        .await
        .and_then(|res| {
            tracing::info!("Successfully got current weather data from external API: {:?}", res);
            Ok(res)
        }
        )
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