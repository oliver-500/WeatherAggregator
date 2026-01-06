

use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use secrecy::ExposeSecret;
use crate::org::unibl::etf::configuration::settings::{ProviderSettings};
use crate::org::unibl::etf::model::errors::openweather_api_error::{OpenWeatherAPIError};
use crate::org::unibl::etf::model::errors::adapter_service_error::{AdapterServiceError};
use crate::org::unibl::etf::model::responses::openweather_current_weather_response::{OpenWeatherAPICurrentWeatherResponse};
use crate::org::unibl::etf::repositories::provider_repository::ProviderRepository;

#[derive(Debug)]
pub struct CurrentWeatherService {
    provider_repository: ProviderRepository,
}


impl CurrentWeatherService {
    fn new() -> Self {
        Self {
            provider_repository: ProviderRepository::default(),
        }
    }

    #[tracing::instrument(name = "Check if ratelimit is exceeded function", skip(redis_pool))]
    pub async fn check_if_ratelimit_is_exceeded(
        &self,
        max_number_of_requests_per_30_mins: u64,
        provider_name: &str,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<bool, AdapterServiceError> {
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


    #[tracing::instrument(name = "Get Current Weather Data by Coordinates Service", skip(client, provider_settings))]
    pub async fn get_current_weather_by_coordinates(
        &self,
        (lat, lon): (f64, f64),
        client: &ClientWithMiddleware,
        provider_settings: &ProviderSettings,
        redis_pool: &deadpool_redis::Pool
    ) -> Result<OpenWeatherAPICurrentWeatherResponse, AdapterServiceError> {
        match self.check_if_ratelimit_is_exceeded(
            provider_settings.requests_per_30_mins,
            provider_settings.name.as_str(),
            redis_pool
        ).await {
            Ok(exceeded) => {
                if exceeded {
                    tracing::info!("Ratelimit exceeded for the geocoding provider.");
                    return Err(AdapterServiceError::RateLimitExceeded);
                }
                tracing::info!("Ratelimit not exceeded for the geocoding provider.");
            },
            Err(e) => {
                return Err(e);
            }
        }


        let response = client.get(format!("{}/{}", provider_settings.base_api_url, provider_settings.current_weather_endpoint).as_str())
            .query(&[
                ("lat", &lat.to_string()),
                ("lon", &lon.to_string()),
                ("appid", &provider_settings.api_key.expose_secret().to_string()),
                ("units", &"metric".to_string()),
            ])
            .send()
            .await
            .map_err(|e| AdapterServiceError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            match self.provider_repository.increment_number_of_requests(
                provider_settings.name.as_str(),
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