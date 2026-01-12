use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;

#[async_trait::async_trait]
pub trait CurrentWeatherQuery {
    type NewDataRequest: Clone + Send + Sync;
    type RetrieveCacheRequest: Clone + Send + Sync;
    type StoreCacheRequest:  Clone + Send + Sync;

    fn build_downstream_request(&self) -> Result<Self::NewDataRequest, AggregatorError>;
    fn build_retrieve_cache_request(&self) -> Result<Self::RetrieveCacheRequest, AggregatorError>;
    fn build_store_cache_request(&self, request: &CurrentWeatherResponse, location_names: Vec<String>) -> Result<Self::StoreCacheRequest, AggregatorError>;

    async fn cache_get(
        &self,
        req: &Self::RetrieveCacheRequest,
        client: &ClientWithMiddleware,
        cache_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError>;

    async fn call_provider(
        &self,
        provider: &ProviderSettings,
        req: &Self::NewDataRequest,
        client: &ClientWithMiddleware,
    ) -> Result<CurrentWeatherResponse, AggregatorError>;

    async fn cache_set(
        &self,
        req: &Self::StoreCacheRequest,
        client: &ClientWithMiddleware,
        cache_settings: &CacheServiceSettings,
    ) -> Result<(), AggregatorError>;
}

