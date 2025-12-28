use reqwest::StatusCode;
use crate::org::unibl::etf::configuration::settings::{GeocodingAPISettings};
use crate::org::unibl::etf::model::dto::location_candidate::LocationCandidate;
use crate::org::unibl::etf::model::errors::geocoding_api_error::ExternalGeocodingApiError;
use crate::org::unibl::etf::model::errors::geocoding_service_error::{GeocodingServiceError};
use crate::org::unibl::etf::model::responses::geocoding_api_response::GeocodingAPIResponse;

pub struct GeocodingService {

}

impl GeocodingService {
    fn new() -> Self {
        Self {
        }
    }
    pub async fn geocode_location(
        &self,
        location: &String,
        limit: u16,
        client: &reqwest::Client,
        settings: &GeocodingAPISettings
    ) -> Result<Vec<LocationCandidate>, GeocodingServiceError> {

        let response = client
            .get(settings.endpoint.clone())
            .query(&[
                ("appid", settings.api_key.clone()),
                ("q", location.clone()),
                ("limit", limit.to_string()),
            ])
            .send()
            .await
            .map_err(|e| GeocodingServiceError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            let body_text = response.text().await.map_err(|e| {
                GeocodingServiceError::ServerError(Some(format!("Failed to get Geocoding API success response body text: {}", e)))
            })?;

            let data: Vec<GeocodingAPIResponse> = serde_json::from_str(&body_text)
                .map_err(|e| {
                    GeocodingServiceError::ResponseParsingError(Some(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, body_text
                    )))
                })?;

            if data.is_empty() {
                return Err(GeocodingServiceError::LocationNotFoundError(Some(location.clone())));
            }

            let mut candidates: Vec<LocationCandidate> = Vec::new();
            for geocoding_response in data {
                candidates.push(geocoding_response.try_into().unwrap());
            }

            Ok(candidates)
        }
        else {
            let status = response.status();

            match status {
                StatusCode::NOT_FOUND |
                StatusCode::UNAUTHORIZED |
                StatusCode::TOO_MANY_REQUESTS |
                StatusCode::BAD_REQUEST => {
                    let error_body_text = response.text().await.map_err(|e| {
                        GeocodingServiceError::ServerError(Some(format!("Failed to get Geocoding API error response body text: {}", e)))
                    })?;

                    let error_body: ExternalGeocodingApiError =
                        serde_json::from_str(&error_body_text)
                            .map_err(|e| {
                                GeocodingServiceError::ResponseParsingError(Some(format!(
                                    "JSON Error: {} | Raw Body: {}",
                                    e, error_body_text
                                )))
                            })?;
                    return Err(GeocodingServiceError::ExternalGeocodingApiError(error_body.cod, Some(error_body.message)));
                },
                _ => {
                    return Err(GeocodingServiceError::ExternalGeocodingApiError(status.as_u16(), None));
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