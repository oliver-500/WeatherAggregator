use reqwest::{Error, StatusCode};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::errors::weatherapi_error_message_response::WeatherAPIErrorMessageResponse;
use crate::org::unibl::etf::model::responses::weatherapi_current_weather_response::WeatherAPICurrentWeatherResponse;

pub struct WeatherAPIService {

}

impl WeatherAPIService {
    fn new() -> Self {
        Self {}
    }
    pub async fn get_current_weather_data_by_coordinates(
        &self,
        (lat, lon): (f64, f64),
        client: &reqwest::Client,
        base_api_url: &str,
        endpoint: &str,
        api_key: &str,
    ) -> Result<WeatherAPICurrentWeatherResponse, AdapterError> {

        let response = client.get(format!("{}/{}", base_api_url, endpoint))
            .query(&[
                ("q", format!("{},{}", &lat.to_string(), &lon.to_string())),
                ("key", api_key.to_string()), // API key is placed here
            ])
            .send()
            .await
            .map_err(|e : Error| {
                AdapterError::ConnectionError(e.to_string())
            })?; //network connection errors


        if response.status().is_success() {
            // 1. Get the raw text first
            let body_text = response.text().await.map_err(|e| {
                AdapterError::ConnectionError(format!("Failed to get body text: {}", e))
            })?;

            // 2. Parse manually using serde_json
            let data: WeatherAPICurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
                    AdapterError::ApiResponseParsingError(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, body_text
                    ))
                })?;

            Ok(data)
        } else {
            let status = response.status();

            match status {
                StatusCode::NOT_FOUND | StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    let error_body_text = response.text().await.map_err(|e| {
                        AdapterError::ConnectionError(format!("Failed to get error body text: {}", e))
                    })?;

                    let error_body: WeatherAPIErrorMessageResponse = serde_json::from_str(&error_body_text)
                        .map_err(|e| {
                            // This 'e' will now contain the EXACT field and line number
                            AdapterError::ApiResponseParsingError(format!(
                                "JSON Error: {} | Raw Body: {}",
                                e, error_body_text
                            ))
                        })?;
                    
                    // Return your own custom error message using the API's "message" field
                    Err(AdapterError::ExternalApiError(error_body.error.code, error_body.error.message))
                },

                _ => {
                    Err(AdapterError::ExternalApiError(status.as_u16(), "Unknown external API error".to_string()))
                }
            }

            // 3. Handle the 400 (or other) error cases

        }

    }



}

impl Default for WeatherAPIService {
    fn default() -> Self {
        Self::new()
    }
}