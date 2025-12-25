use reqwest::StatusCode;
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::errors::ambiguous_location_error::Candidate;
use crate::org::unibl::etf::model::errors::openweatherapi_error_message_response::OpenWeatherAPIErrorMessageResponse;
use crate::org::unibl::etf::model::responses::geocoding_response::GeocodingResponse;

pub struct GeocodingService {

}

impl GeocodingService {

    fn new() -> Self {
        Self {
        }
    }
    pub async fn geocode_location(
        &self,
        location: &str,
        client: &reqwest::Client,
        limit: u8
    ) -> Result<(f64, f64), AdapterError> {
        let response = client
            .get("https://api.openweathermap.org/geo/1.0/direct")
            .query(&[
                ("appid", "0a822e11245fcf3d67384f9c71ec9a84"),
                ("q", location),
                ("limit", limit.to_string().as_str()),
            ])
            .send()
            .await
            .map_err(|e| AdapterError::ConnectionError(e.to_string()))?;

        if response.status().is_success() {
            let body_text = response.text().await.map_err(|e| {
                AdapterError::ServerError(Some(format!("Failed to get response body text: {}", e)))
            })?;

            let data: Vec<GeocodingResponse> = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AdapterError::ApiResponseParsingError(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, body_text
                    ))
                })?;

            if data.len() == 0 {
                return Err(AdapterError::LocationNotFound(location.to_string()));
            } else if data.len() == 1 {
                let geocoding_response = data.get(0).unwrap();
                return Ok((geocoding_response.lat, geocoding_response.lon));
            } else {
                let mut candidates: Vec<Candidate> = Vec::new();
                for geocoding_response in data {
                    candidates.push(geocoding_response.try_into().unwrap());
                }
                return Err(AdapterError::AmbiguousLocation(candidates));
            }
        }
        else {
            let status = response.status();

            match status {
                StatusCode::NOT_FOUND |
                StatusCode::UNAUTHORIZED |
                StatusCode::TOO_MANY_REQUESTS |
                StatusCode::BAD_REQUEST => {
                    let error_body_text = response.text().await.map_err(|e| {
                        AdapterError::ServerError(Some(format!("Failed to get error body text: {}", e)))
                    })?;

                    let error_body: OpenWeatherAPIErrorMessageResponse =
                        serde_json::from_str(&error_body_text)
                            .map_err(|e| {
                                // This 'e' will now contain the EXACT field and line number
                                AdapterError::ApiResponseParsingError(format!(
                                    "JSON Error: {} | Raw Body: {}",
                                    e, error_body_text
                                ))
                            })?;
                    return Err(AdapterError::ExternalApiError(error_body.cod, Some(error_body.message)));
                },
                _ => {
                    return Err(AdapterError::ExternalApiError(status.as_u16(), None));
                }
            }
        }
    }
}

impl Default for GeocodingService {
    fn default() -> Self {
        Self::new()
    }
}