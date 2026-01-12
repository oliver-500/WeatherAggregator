use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::AdapterServiceError;
use crate::org::unibl::etf::model::requests::current_weather_query::CurrentWeatherQuery;
use crate::org::unibl::etf::model::requests::downstream_current_weather_request::DownstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::requests::retrieve_current_weather_cache_request::RetrieveCurrentWeatherCacheRequest;
use crate::org::unibl::etf::model::requests::store_current_weather_cache_request::{StoreCurrentWeatherCacheRequest, };
use crate::org::unibl::etf::model::requests::upstream_current_weather_request_by_location::UpstreamCurrentWeatherRequestByLocation;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_cache_service::CurrentWeatherCacheService;

#[derive(Debug, Clone)]
pub struct CurrentWeatherLocationQuery {
    pub request: UpstreamCurrentWeatherRequestByLocation,
    pub cache_service: CurrentWeatherCacheService,
}

#[async_trait::async_trait]
impl CurrentWeatherQuery for CurrentWeatherLocationQuery {
    type NewDataRequest = DownstreamCurrentWeatherRequest;

    type RetrieveCacheRequest = RetrieveCurrentWeatherCacheRequest;
    type StoreCacheRequest = StoreCurrentWeatherCacheRequest;

    fn build_downstream_request(&self) -> Result<Self::NewDataRequest, AggregatorError> {
        DownstreamCurrentWeatherRequest::try_from(&self.request)
            .map_err(|e| AggregatorError::ServerError(Some(e.to_string())))
    }

    fn build_retrieve_cache_request(&self) -> Result<Self::RetrieveCacheRequest, AggregatorError> {
        Ok(RetrieveCurrentWeatherCacheRequest {
            location_name: Some(self.request.location_name.clone()),
            lat: None,
            lon: None,
            country: self.request.country.clone(),
            state: self.request.state.clone(),
        })
    }

    fn build_store_cache_request(&self, current_weather_response: &CurrentWeatherResponse, location_names: Vec<String>) -> Result<Self::StoreCacheRequest, AggregatorError> {
        Ok(StoreCurrentWeatherCacheRequest {
            lat: current_weather_response.location.clone().lat,
            lon: current_weather_response.location.clone().lon,
            current_weather_data: current_weather_response.clone(),
            location_names: location_names,
        })
    }

    async fn cache_get(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        client: &ClientWithMiddleware,
        cache_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        self.cache_service.get_cached_current_weather_data_by_location(req, client, cache_settings).await
    }

    async fn call_provider(
        &self,
        provider: &ProviderSettings,
        req: &DownstreamCurrentWeatherRequest,
        client: &ClientWithMiddleware,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        let url = format!("{}://{}:{}/api/v1/current_weather", provider.scheme, provider.host, provider.port);

        let response = client
            .get(url)
            .query(&[("location_name", req.location_name.clone())])
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            let body_text = response.text()
                .await
                .map_err(|e| AggregatorError::ServerError(
                    Some(format!("Failed to get {} Adapter Service success response body text: {}", provider.name, e))
                ))?;

            let res: CurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AggregatorError::ResponseParsingError(Some(format!(
                        "Failed to parse {} Adapter Service success response: JSON Error: {} | Raw Body: {}",
                        provider.name, e, body_text
                    )))
                })?;

            return Ok(res);
        } else {
            let error_body_text = response.text().await.map_err(|e| {
                AggregatorError::ServerError(
                    Some(format!("Failed to get {} Adapter Service error body text: {}", provider.name, e))
                )
            })?;

            let error_body: AdapterServiceError = serde_json::from_str(&error_body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
                    AggregatorError::ResponseParsingError(Some(format!(
                        "Failed to parse {} Adapter Service success response: JSON Error: {} | Raw Body: {}",
                        provider.name, e, error_body_text
                    )))
                })?;

            Err(AggregatorError::from(error_body.error.code))
        }

    }

    async fn cache_set(
        &self,
        req: &StoreCurrentWeatherCacheRequest,
        client: &ClientWithMiddleware,
        cache_settings: &CacheServiceSettings,
    ) -> Result<(), AggregatorError> {
        self.cache_service.save_current_weather_data_to_cache(client, cache_settings, &req).await
    }
}
