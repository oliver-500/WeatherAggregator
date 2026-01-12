use reqwest::Response;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::CacheServiceSettings;
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;

use crate::org::unibl::etf::model::requests::retrieve_current_weather_cache_request::RetrieveCurrentWeatherCacheRequest;
use crate::org::unibl::etf::model::requests::store_current_weather_cache_request::{StoreCurrentWeatherCacheRequest, };


use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;


#[derive(Debug, Clone)]
pub struct CurrentWeatherCacheService {

}

impl CurrentWeatherCacheService {
    fn new() -> Self {
        Self {}
    }

    #[tracing::instrument(name = "Get Cached Current Weather Data Function", skip(client, cache_service_settings))]
    pub async fn get_cached_current_weather_data_by_coordinates(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        client: &ClientWithMiddleware,
        cache_service_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        let url = format!("{}://{}:{}/api/v1/current_weather_by_coordinates", cache_service_settings.scheme, cache_service_settings.host, cache_service_settings.port);

        let lat = match req.lat {
            Some(lat) => lat,
            None => return Err(AggregatorError::ServerError(Some("Latitude not provided".to_string()))),
        };

        let lon = match req.lon {
            Some(lat) => lat,
            None => return Err(AggregatorError::ServerError(Some("Longitude not provided".to_string()))),
        };

        let mut params = Vec::new();
        params.push(("lat", lat));
        params.push(("lon", lon));

        let response = client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

        self.process_cache_service_get_cached_current_weather_data_response(response).await
    }

    #[tracing::instrument(name = "Get Cached Current Weather Data Function", skip(client, cache_service_settings))]
    pub async fn get_cached_current_weather_data_by_location(
       &self,
       req: &RetrieveCurrentWeatherCacheRequest,
       client: &ClientWithMiddleware,
       cache_service_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        let url = format!("{}://{}:{}/api/v1/current_weather_by_location", cache_service_settings.scheme, cache_service_settings.host, cache_service_settings.port);

        let location_name = match &req.location_name {
            Some(location_name) => location_name.clone(),
            None => return Err(AggregatorError::ServerError(Some("Location name not provided".to_string()))),
        };

        println!("joj");
        let mut params = Vec::new();
        params.push(("location_name", location_name));

        if req.country.is_some() {
            params.push(("country", req.country.clone().unwrap()));
        }
        if req.state.is_some() {
            params.push(("state", req.state.clone().unwrap()));
        }

        let response = client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

        self.process_cache_service_get_cached_current_weather_data_response(response).await
    }

    #[tracing::instrument(name = "Send Current Weather Data to Cache Service function", skip(client, cache_service_settings))]
    pub async fn save_current_weather_data_to_cache(
        &self,
        client: &ClientWithMiddleware,
        cache_service_settings: &CacheServiceSettings,
        data: &StoreCurrentWeatherCacheRequest,
    ) -> Result<(), AggregatorError> {
        let url = format!("{}://{}:{}/api/v1/current_weather", cache_service_settings.scheme, cache_service_settings.host, cache_service_settings.port);

        let response = client
            .put(url)
            .json(&data)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

        self.process_cache_service_save_current_weather_cache_data_response(response).await
    }

    pub async fn process_cache_service_get_cached_current_weather_data_response(
        &self,
        response: Response
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        if response.status().is_success() {
            let body_text = response.text()
                .await
                .map_err(|e| AggregatorError::ServerError(
                    Some(format!("Failed to get Cache Service success response body text: {}", e))
                ))?;

            let res: CurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AggregatorError::ResponseParsingError(Some(format!(
                        "Failed to parse Cache Service success response: JSON Error: {} | Raw Body: {}",
                        e, body_text
                    )))
                })?;

            Ok(res)
        } else {
            let error_body_text = response.text().await.map_err(|e| {
                AggregatorError::ServerError(Some(format!("Failed to get Cache Service error response body text: {}", e)))
            })?;

            let error_body: CacheServiceError = serde_json::from_str(&error_body_text)
                .map_err(|e| {
                    AggregatorError::ResponseParsingError(Some(format!(
                        "Failed to parse Cache Service error response. JSON Error: {} | Raw Body: {}",
                        e, error_body_text
                    )))
                })?;

            tracing::info!("Cache Service Error while trying to get current weather cached data with error response: {:?}", error_body);

            Err(AggregatorError::from(error_body.error.code))
        }
    }

    pub async fn process_cache_service_save_current_weather_cache_data_response(
        &self,
        response: Response
    ) -> Result<(), AggregatorError> {
        if response.status().is_success() {
            Ok(())
        } else {
            let error_body_text = response.text().await.map_err(|e| {
                AggregatorError::ServerError(Some(format!("Failed to get error body text: {}", e)))
            })?;

            let error_body: CacheServiceError = serde_json::from_str(&error_body_text)
                .map_err(|e| {
                    AggregatorError::ResponseParsingError(Some(format!(
                        "Error parsing Cache Service error response: JSON Error: {} | Raw Body: {}",
                        e, error_body_text
                    )))
                })?;
            tracing::error!("Cache Service Error while trying to save current weather data to cache with error response: {:?}", error_body);

            Err(AggregatorError::from(error_body.error.code))
        }
    }

}


impl Default for CurrentWeatherCacheService {
    fn default() -> Self {
        Self::new()
    }
}