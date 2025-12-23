use std::collections::HashMap;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use reqwest::Client;
use crate::org::unibl::etf::configuration::settings::ProviderSettings;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails, HttpMappedError};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::requests::weather_data_request::{CurrentWeatherRequest};
use crate::org::unibl::etf::model::responses::unified_current_weather_response::UnifiedCurrentWeatherResponse;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;
use crate::org::unibl::etf::services::weather_api_service::WeatherAPIService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/weatherapi").route(web::post().to(get_weather_api_current_weather_data)));
}

async fn get_weather_api_current_weather_data(
    weather_api_service: web::Data<WeatherAPIService>,
    request: web::Json<CurrentWeatherRequest>,
    http_client: web::Data<Client>,
    providers_settings: web::Data<HashMap<String, ProviderSettings>>,
    geocoding_service: web::Data<GeocodingService>,
) -> impl Responder {
    let Some(settings) = providers_settings.get("weatherapi.com") else {//skratiti kod
        let generic_service_error_details = GenericServiceErrorDetails {
            code: AdapterError::ServerError,
            code_numeric: 500,
            message: "No configuration found for this provider".to_string(),
            provider: "weatherapi.com".to_string(),
            timestamp: Utc::now(), //mozda lokal time
        };
        let e = GenericServiceError {
            error: generic_service_error_details,
        };

        return e.to_response();
    };

    let search_by_location : bool = request.lon.is_none() || request.lat.is_none();

    let (lat, lon) = if search_by_location {
        match geocoding_service.geocode_location(
            request.location.clone().unwrap_or("".to_string()).as_str(),
            http_client.get_ref(),
            request.limit
        ).await {
            Ok((lat, lon)) => (lat, lon),
            Err(e) => {
                let generic_service_error_details = GenericServiceErrorDetails {
                    code: e.clone(),
                    code_numeric: e.as_numeric(),
                    message: e.get_message(),
                    provider: "weatherapi.com".to_string(),
                    timestamp: Utc::now(), //mozda lokal time
                };
                let e = GenericServiceError {
                    error: generic_service_error_details,
                };

                return e.to_response();
            }
        }
    } else {
        (request.lat.unwrap(), request.lon.unwrap())
    };

    let res = match weather_api_service
        .get_current_weather_data_by_coordinates(
            (lat, lon),
            http_client.get_ref(),
            settings.base_api_url.as_str(),
            settings.current_weather_endpoint.as_str(),
            settings.api_key.as_str(),
        )
        .await {
            Ok(res) => res,
            Err(e) => {
                let generic_service_error_details = GenericServiceErrorDetails {
                    code: e.clone(),
                    code_numeric: e.as_numeric(),
                    message: e.get_message(),
                    provider: settings.name.clone(),
                    timestamp: Utc::now(), //mozda lokal time
                };
                let e = GenericServiceError {
                    error: generic_service_error_details,
                };

                return e.to_response();
            }
        };

    match UnifiedCurrentWeatherResponse::try_from(res) {
        Ok(weather_data) => HttpResponse::Ok().json(weather_data),
        Err(e) => {
            let generic_service_error_details = GenericServiceErrorDetails {
                code: e.clone(),
                code_numeric: e.as_numeric(),
                message: e.get_message(),
                provider: "weatherapi.com".to_string(),
                timestamp: Utc::now(), //mozda lokal time
            };
            let e = GenericServiceError {
                error: generic_service_error_details,
            };

            return e.to_response();
        },
    }
}