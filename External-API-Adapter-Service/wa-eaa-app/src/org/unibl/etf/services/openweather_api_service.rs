use reqwest::StatusCode;
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::errors::openweatherapi_error_message_response::OpenWeatherAPIErrorMessageResponse;
use crate::org::unibl::etf::model::responses::openweatherapi_current_weather_response::OpenWeatherAPICurrentWeatherResponse;

pub struct OpenWeatherAPIService {

}

impl OpenWeatherAPIService {

    fn new() -> Self {
        Self {
        }
    }

    pub async fn get_current_weather_data_by_coordinates(
        &self,
        (lat, lon): (f64, f64),
        client: &reqwest::Client,
        base_api_url: &str,
        endpoint: &str,
        api_key: &str,
    ) -> Result<OpenWeatherAPICurrentWeatherResponse, AdapterError> {


        let response = client.get(format!("{}/{}", base_api_url, endpoint).as_str())
            .query(&[
                ("lat", &lat.to_string()),
                ("lon", &lon.to_string()),
                ("appid", &api_key.to_string()), // API key is placed here
                ("units", &"metric".to_string()), // Optional: get temperature in Celsius
            ])
            .send()
            .await
            .map_err(|e| AdapterError::ConnectionError(e.to_string()))?;

        if response.status().is_success() {
            // 1. Get the raw text first
            let body_text = response.text().await.map_err(|e| {
                AdapterError::ConnectionError(format!("Failed to get body text: {}", e))
            })?;

            // 2. Parse manually using serde_json
            let data: OpenWeatherAPICurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
                    AdapterError::ApiResponseParsingError(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, body_text
                    ))
                })?;

            Ok(data)
        }
        else {
            let status = response.status();

            match status {
                StatusCode::INTERNAL_SERVER_ERROR => {
                    return Err(AdapterError::ExternalApiError(500, String::from("Unknown External API Server Error")));
                },
                StatusCode::NOT_FOUND | StatusCode::UNAUTHORIZED | StatusCode::TOO_MANY_REQUESTS | StatusCode::BAD_REQUEST => {

                    let error_body_text = response.text().await.map_err(|e| {
                        AdapterError::ConnectionError(format!("Failed to get error body text: {}", e))
                    })?;

                    let error_body: OpenWeatherAPIErrorMessageResponse = serde_json::from_str(&error_body_text)
                        .map_err(|e| {
                            // This 'e' will now contain the EXACT field and line number
                            AdapterError::ApiResponseParsingError(format!(
                                "JSON Error: {} | Raw Body: {}",
                                e, error_body_text
                            ))
                        })?;


                    return Err(AdapterError::ExternalApiError(error_body.cod, error_body.message));
                },
                _ => {
                    return Err(AdapterError::ExternalApiError(status.as_u16(), String::from("Unexpected External API Error")));
                }
            }



        }
    }




}

impl Default for OpenWeatherAPIService {
    fn default() -> Self {
        Self::new()
    }
}