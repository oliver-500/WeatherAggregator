
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use secrecy::ExposeSecret;
use crate::org::unibl::etf::configuration::settings::{Settings};

use crate::org::unibl::etf::model::errors::openweather_api_error::{OpenWeatherAPIError};
use crate::org::unibl::etf::model::errors::adapter_service_error::{AdapterServiceError};
use crate::org::unibl::etf::model::requests::current_weather_request::CurrentWeatherRequest;

use crate::org::unibl::etf::model::responses::openweather_current_weather_response::{OpenWeatherAPICurrentWeatherResponse};
use crate::org::unibl::etf::model::responses::uniform_current_weather_response::UniformCurrentWeatherResponse;
use crate::org::unibl::etf::repositories::provider_repository::ProviderRepository;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;

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


    #[tracing::instrument(name = "Get Current Weather Data by Coordinates Service", skip(client, settings))]
    pub async fn get_current_weather(
        &self,
        req: CurrentWeatherRequest,
        client: &ClientWithMiddleware,
        settings: &Settings,
        redis_pool: &deadpool_redis::Pool,
        geocoding_service: &GeocodingService
    ) -> Result<UniformCurrentWeatherResponse, AdapterServiceError> {
        match self.check_if_ratelimit_is_exceeded(
            settings.provider.requests_per_30_mins,
            settings.provider.name.as_str(),
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


        let candidate = if req.location_name.is_some() {
            let candidate = match geocoding_service.geocode_location(
                req.location_name.clone().unwrap_or("".to_string()).as_str(),
                client,
                5,
                &settings.geocoding_service
            ).await {
                Ok(candidate) => {
                    tracing::info!("Successfully geocoded location. Result: {:?}", candidate);
                    candidate
                },
                Err(e) => return {
                    tracing::error!("Could not geocode location with error: {:?}", e);
                    Err(e)
                },
            };
            Some(candidate)
        } else {
            None
        };

        let (lat, lon) = match &candidate {
            Some(cand) => (cand.lat, cand.lon),
            None => (req.lat.unwrap(), req.lon.unwrap()) //validation done earlier
        };

        let response = client.get(format!("{}/{}", settings.provider.base_api_url, settings.provider.current_weather_endpoint).as_str())
            .query(&[
                ("lat", &lat.to_string()),
                ("lon", &lon.to_string()),
                ("appid", &settings.provider.api_key.expose_secret().to_string()),
                ("units", &"metric".to_string()),
            ])
            .send()
            .await
            .map_err(|e| AdapterServiceError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            match self.provider_repository.increment_number_of_requests(
                settings.provider.name.as_str(),
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



            let response = UniformCurrentWeatherResponse::try_from(data)
                .and_then(|mut weather_data| {
                    match candidate {
                        Some(cand) => {
                            weather_data.set_state_region_province_or_entity(cand.state.clone());
                        },
                        None => {

                        }
                    }
                    Ok(weather_data)
                })
                .map_err(|e| {
                    tracing::error!("Was not able to get transform weather data to uniform format with error: {:?}", e.get_message());
                    e
                })?;

            Ok(response)
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