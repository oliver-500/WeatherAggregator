use reqwest::StatusCode;
use crate::org::unibl::etf::configuration::settings::GeocodingServiceSettings;

use crate::org::unibl::etf::model::errors::adapter_service_error::AdapterServiceError;

use crate::org::unibl::etf::model::errors::geocoding_error::{GeocodingGenericError};
use crate::org::unibl::etf::model::responses::geocoding_response::GeocodingResponse;

#[derive(Debug)]
pub struct GeocodingService {

}

impl GeocodingService {

    fn new() -> Self {
        Self {
        }
    }

    #[tracing::instrument(name = "Geocode Location Service", skip(client, settings))]
    pub async fn geocode_location(
        &self,
        location: &str,
        client: &reqwest::Client,
        limit: u8,
        settings: &GeocodingServiceSettings,
    ) -> Result<(f64, f64), AdapterServiceError> {
        let response = client
            .get(format!("http://{}:{}/api/v1/geocode", settings.host, settings.port))
            .query(&[
                ("location_name", location),
                ("limit", limit.to_string().as_str()),
            ])
            .send()
            .await
            .map_err(|e| {
                AdapterServiceError::ConnectionError(Some(e.to_string()))
            })?;

        if response.status().is_success() {
            let body_text = response.text().await.map_err(|e| {
                AdapterServiceError::ServerError(Some(e.to_string()))
            })?;

            let data: GeocodingResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AdapterServiceError::GeocodingResponseParsingError(Some(format!(
                        "Failed to parse Geocoding Service success body response. JSON Error: {} | Raw Body: {}",
                        e, body_text
                    )))
                })?;

            if data.candidates.len() == 0 {
                tracing::error!("No candidates for geocoding found in response.");
                return Err(AdapterServiceError::LocationNotFoundError(Some(location.to_string())));
            } else if data.candidates.len() == 1 {
                let geocoding_response = data.candidates.get(0).unwrap();
                return Ok((geocoding_response.lat, geocoding_response.lon));
            } else {
                tracing::info!("Found multiple possible geocoding candidates for location found in request.");
                return Err(AdapterServiceError::AmbiguousLocationNameError(data.candidates));
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
                        AdapterServiceError::ServerError(Some(e.to_string()))
                    })?;

                    let error_body: GeocodingGenericError =
                        serde_json::from_str(&error_body_text)
                            .map_err(|e| {

                                AdapterServiceError::GeocodingResponseParsingError(Some(format!(
                                    "Failed to parse Geocoding Service error response body. JSON Error: {} | Raw Body: {}",
                                    e, error_body_text
                                )))
                            })?;
                    tracing::error!("Geocoding Service while trying to geocode location");

                    return Err(AdapterServiceError::from(error_body.error.code));
                },
                _ => {
                    return Err(AdapterServiceError::GeocodingServiceError(status.as_u16(), None));
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