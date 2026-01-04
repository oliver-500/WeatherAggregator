use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{ProviderSettings};
use crate::org::unibl::etf::model::errors::openweather_api_error::{OpenWeatherAPIError};
use crate::org::unibl::etf::model::errors::adapter_service_error::{AdapterServiceError};
use crate::org::unibl::etf::model::responses::openweather_current_weather_response::{OpenWeatherAPICurrentWeatherResponse};

#[derive(Debug)]
pub struct CurrentWeatherService {

}


impl CurrentWeatherService {
    fn new() -> Self {
        Self {
        }
    }

    #[tracing::instrument(name = "Get Current Weather Data by Coordinates Service", skip(client, settings))]
    pub async fn get_current_weather_by_coordinates(
        &self,
        (lat, lon): (f64, f64),
        client: &ClientWithMiddleware,
        settings: &ProviderSettings
    ) -> Result<OpenWeatherAPICurrentWeatherResponse, AdapterServiceError> {
        let response = client.get(format!("{}/{}", settings.base_api_url, settings.current_weather_endpoint).as_str())
            .query(&[
                ("lat", &lat.to_string()),
                ("lon", &lon.to_string()),
                ("appid", &settings.api_key.to_string()),
                ("units", &"metric".to_string()),
            ])
            .send()
            .await
            .map_err(|e| AdapterServiceError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            let body_text = response.text().await.map_err(|e| {
                AdapterServiceError::ServerError(Some(format!("Failed to get successful external API response body text: {}", e)))
            })?;

            let data: OpenWeatherAPICurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
                    AdapterServiceError::ExternalAPIResponseParsingError(Some(format!(
                        "Error while parsing successful external API response body text. JSON Error: {} | Raw Body: {}",
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
                        AdapterServiceError::ServerError(Some(format!("Failed to get external API error response  body text: {}", e)))
                    })?;

                    let error_body: OpenWeatherAPIError = serde_json::from_str(&error_body_text)
                        .map_err(|e| {
                            AdapterServiceError::ExternalAPIResponseParsingError(Some(format!(
                                "Error while parsing external API error response body text. JSON Error: {} | Raw Body: {}",
                                e, error_body_text
                            )))
                        })?;
                    tracing::error!("Error while calling External API: {:?}", error_body);
                    return Err(AdapterServiceError::OpenWeatherAPIError(error_body.cod, Some(error_body.message)));
                },
                _ => {
                    return Err(AdapterServiceError::OpenWeatherAPIError(status.as_u16(), None));
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