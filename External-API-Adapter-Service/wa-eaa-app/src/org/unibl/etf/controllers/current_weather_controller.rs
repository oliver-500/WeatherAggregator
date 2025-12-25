use std::collections::HashMap;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use crate::org::unibl::etf::configuration::settings::ProviderSettings;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::requests::weather_data_request::{CurrentWeatherRequest};
use crate::org::unibl::etf::model::responses::unified_current_weather_response::UnifiedCurrentWeatherResponse;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;
use crate::org::unibl::etf::services::openweather_api_service::OpenWeatherAPIService;
use crate::org::unibl::etf::services::weather_api_service::WeatherAPIService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/currentweather").route(web::post().to(get_current_weather_data)));
}

async fn get_current_weather_data(
    openweather_api_service: web::Data<OpenWeatherAPIService>,
    weather_api_service: web::Data<WeatherAPIService>,
    request: web::Json<CurrentWeatherRequest>,
    http_client: web::Data<Client>,
    providers_settings: web::Data<HashMap<String, ProviderSettings>>,
    geocoding_service: web::Data<GeocodingService>,
) -> Result<impl Responder, GenericServiceError> {

    let settings = providers_settings
        .get(request.provider.as_str())
        .ok_or_else(|| GenericServiceError {
            error: GenericServiceErrorDetails::new_config_error(request.provider.as_str())
        })?;

    let search_by_location: bool = request.lon.is_none() || request.lat.is_none();

    let (lat, lon) = if search_by_location {
        geocoding_service.geocode_location(
            request.location.as_deref().unwrap_or(""),
            http_client.get_ref(),
            request.limit
        ).await.map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), e)
        })?
    } else {
        (request.lat.unwrap(), request.lon.unwrap())
    };

    if request.provider == "openweathermap.org" {
         let res = openweather_api_service
            .get_current_weather_data_by_coordinates(
                (lat, lon),
                http_client.get_ref(),
                settings.base_api_url.as_str(),
                settings.current_weather_endpoint.as_str(),
                settings.api_key.as_str(),
            ).await.map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), e)
        })?;

        Ok(UnifiedCurrentWeatherResponse::try_from(res)
            .and_then(|weather_data| Ok(HttpResponse::Ok().json(weather_data)))
            .map_err(|e| GenericServiceError {
                error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), e)
            })?
        )
    }
    else if request.provider == "weatherapi.com" {
        let res = weather_api_service
            .get_current_weather_data_by_coordinates(
                (lat, lon),
                http_client.get_ref(),
                settings.base_api_url.as_str(),
                settings.current_weather_endpoint.as_str(),
                settings.api_key.as_str(),
            ).await.map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), e)
        })?;

        Ok(UnifiedCurrentWeatherResponse::try_from(res)
            .and_then(|weather_data| Ok(HttpResponse::Ok().json(weather_data)))
            .map_err(|e| GenericServiceError {
                error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), e)
            })?
        )
    }
    else {
        Err(GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(settings.name.as_str(), AdapterError::UnknownProvider)
        })
    }

}