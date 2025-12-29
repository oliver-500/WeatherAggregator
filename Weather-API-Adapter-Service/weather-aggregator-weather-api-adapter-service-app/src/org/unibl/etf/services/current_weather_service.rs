use reqwest::{Error, StatusCode};
use crate::org::unibl::etf::configuration::settings::{ProviderSettings};
use crate::org::unibl::etf::model::errors::weather_api_error::{WeatherAPIError};
use crate::org::unibl::etf::model::errors::adapter_service_error::{AdapterServiceError};
use crate::org::unibl::etf::model::requests::current_weather_request::CurrentWeatherRequest;
use crate::org::unibl::etf::model::responses::weatherapi_current_weather_response::{WeatherAPICurrentWeatherResponse};

pub struct CurrentWeatherService {

}

impl CurrentWeatherService {
    fn new() -> Self {
        Self {
        }
    }
    pub async fn get_current_weather_by_coordinates_or_location_name(
        &self,
        request: &CurrentWeatherRequest,
        client: &reqwest::Client,
        settings: &ProviderSettings
    ) -> Result<WeatherAPICurrentWeatherResponse, AdapterServiceError> {

        let q_argument = if request.lat.is_none() || request.lon.is_none() {
            request.location_name.clone().unwrap_or("".to_string())
        }
        else {
            format!("{},{}", &request.lat.clone().unwrap().to_string(), &request.lon.clone().unwrap().to_string())
        };

        let response = client.get(format!("{}/{}", settings.base_api_url, settings.current_weather_endpoint))
            .query(&[
                ("q", q_argument.as_str()),
                ("key", settings.api_key.as_str()),
            ])
            .send()
            .await
            .map_err(|e : Error| {
                AdapterServiceError::ConnectionError(Some(e.to_string()))
            })?;

        if response.status().is_success() {
            let body_text = response.text().await.map_err(|e| {
                AdapterServiceError::ServerError(Some(format!("Failed to get body text: {}", e)))
            })?;

            let data: WeatherAPICurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AdapterServiceError::ExternalAPIResponseParsingError(Some(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, body_text
                    )))
                })?;

            Ok(data)
        }
        else {
            let status = response.status();

            match status {
                StatusCode::NOT_FOUND |
                StatusCode::UNAUTHORIZED |
                StatusCode::TOO_MANY_REQUESTS |
                StatusCode::BAD_REQUEST => {

                    let error_body_text = response.text().await.map_err(|e| {
                        AdapterServiceError::ServerError(Some(format!("Failed to get error body text: {}", e)))
                    })?;

                    let error_body: WeatherAPIError = serde_json::from_str(&error_body_text)
                        .map_err(|e| {
                            AdapterServiceError::ExternalAPIResponseParsingError(Some(format!(
                                "JSON Error: {} | Raw Body: {}",
                                e, error_body_text
                            )))
                        })?;
                    if error_body.error.code == 1006 {
                        return Err(AdapterServiceError::LocationNotFoundError(
                            request.location_name.clone().unwrap())
                        );
                    }
                    return Err(AdapterServiceError::WeatherAPIError(error_body.error.code, Some(error_body.error.message)));
                },
                _ => {
                    return Err(AdapterServiceError::WeatherAPIError(status.as_u16(), None));
                }
            }
        }
    }
}

impl Default for CurrentWeatherService {
    fn default() -> Self {
        Self::new()
    }
}