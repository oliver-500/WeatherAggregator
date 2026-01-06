use reqwest::StatusCode;
use secrecy::ExposeSecret;
use crate::org::unibl::etf::configuration::settings::{GeocodingAPISettings};
use crate::org::unibl::etf::model::dto::location_candidate::LocationCandidate;
use crate::org::unibl::etf::model::errors::geocoding_api_error::ExternalGeocodingApiError;
use crate::org::unibl::etf::model::errors::geocoding_service_error::{GeocodingServiceError};
use crate::org::unibl::etf::model::responses::geocoding_api_response::GeocodingAPIResponse;
use crate::org::unibl::etf::repositories::provider_repository::ProviderRepository;

#[derive(Debug)]
pub struct GeocodingService {
    provider_repository: ProviderRepository,
}

impl GeocodingService {
    fn new() -> Self {
        Self {
            provider_repository: ProviderRepository::default()
        }
    }

    #[tracing::instrument(name = "Check if ratelimit is exceeded function", skip(redis_pool))]
    pub async fn check_if_ratelimit_is_exceeded(
        &self,
        max_number_of_requests_per_30_mins: u64,
        provider_name: &str,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<bool, GeocodingServiceError> {
        match self.provider_repository.get_number_of_requests(provider_name, redis_pool).await{
            Ok(number_of_requests) => {
                tracing::info!("Successfully retrieved number of requests made for the geocoding provider");
                Ok(number_of_requests >= max_number_of_requests_per_30_mins as i64)
            },
            Err(e) => {
                tracing::info!("Was not able to get number of requests made for the geocoding provider: {}", e.get_message());
                Err(e)
            },
        }
    }

    #[tracing::instrument(name = "Geocode Location Service", skip(client, settings))]
    pub async fn geocode_location(
        &self,
        location: &String,
        limit: u16,
        client: &reqwest::Client,
        settings: &GeocodingAPISettings,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<Vec<LocationCandidate>, GeocodingServiceError> {
        match self.check_if_ratelimit_is_exceeded(
            settings.requests_per_30_mins,
            settings.provider.as_str(),
            redis_pool
        ).await {
            Ok(exceeded) => {
                if exceeded {
                    tracing::info!("Ratelimit exceeded for the geocoding provider.");
                    return Err(GeocodingServiceError::RateLimitExceeded);
                }
                tracing::info!("Ratelimit not exceeded for the geocoding provider.");
            },
            Err(e) => {
                return Err(e);
            }
        }

        let response = client
            .get(settings.endpoint.clone())
            .query(&[
                ("appid", settings.api_key.expose_secret().clone()),
                ("q", location.clone()),
                ("limit", limit.to_string()),
            ])
            .send()
            .await
            .map_err(|e| GeocodingServiceError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            match self.provider_repository.increment_number_of_requests(
                settings.provider.as_str(),
                redis_pool,
            ).await {
                Ok(new_value) => {
                    tracing::info!("Successfully incremented number of requests made. New value: {}", new_value)
                },
                Err(e) => {
                    tracing::error!("Failed to increment number of requests made with error: {:?}", e.get_message())
                }
            }


            let body_text = response.text().await.map_err(|e| {
                GeocodingServiceError::ServerError(Some(format!("Failed to get Geocoding API success response body text: {}", e)))
            })?;

            let data: Vec<GeocodingAPIResponse> = serde_json::from_str(&body_text)
                .map_err(|e| {
                    GeocodingServiceError::ResponseParsingError(Some(format!(
                        "Failed to parse Geocoding API success response body text. JSON Error: {} | Raw Body: {}",
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
                                    "Failed to parse Geocoding API error response body text. JSON Error: {} | Raw Body: {}",
                                    e, error_body_text
                                )))
                            })?;
                    tracing::error!("Geocoding API error while geocoding location: {:?}", error_body);
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