use std::net::IpAddr;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::AdapterServiceError;
use crate::org::unibl::etf::model::requests::current_weather_query::CurrentWeatherQuery;
use crate::org::unibl::etf::model::requests::downstream_current_weather_request::DownstreamCurrentWeatherRequest;

use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;

#[derive(Debug, Clone)]
pub struct CurrentWeatherIpAddressQuery {
    pub request: IpAddr,
}


#[async_trait::async_trait]
impl CurrentWeatherQuery for CurrentWeatherIpAddressQuery {
    type NewDataRequest = DownstreamCurrentWeatherRequest;

    type RetrieveCacheRequest = ();
    type StoreCacheRequest = ();

    fn build_downstream_request(&self) -> Result<Self::NewDataRequest, AggregatorError> {
        DownstreamCurrentWeatherRequest::try_from(&self.request)
            .map_err(|e| AggregatorError::ServerError(Some(e.to_string())))
    }

    fn build_retrieve_cache_request(&self) -> Result<Self::RetrieveCacheRequest, AggregatorError> {
        Err(AggregatorError::ServerError(None))
    }

    fn build_store_cache_request(&self, _current_weather_response: &CurrentWeatherResponse, _location_names: Vec<String>) -> Result<Self::StoreCacheRequest, AggregatorError> {
        Err(AggregatorError::ServerError(None))
    }

    async fn cache_get(
        &self,
        _req: &(),
        _client: &ClientWithMiddleware,
        _cache_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        Err(AggregatorError::StoringCacheError(Some("Cache by IP address not supported.".to_string())))
    }

    async fn call_provider(
        &self,
        provider: &ProviderSettings,
        req: &DownstreamCurrentWeatherRequest,
        client: &ClientWithMiddleware,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        if provider.ip_support == false {
            return Err(AggregatorError::IpLookupNotSupported);
        }
        let url = format!("{}://{}:{}/api/v1/current_weather_by_ip_address", provider.scheme, provider.host, provider.port);

        let response = client
            .get(url)
            .query(&[("ip_address", req.ip_address.clone())])
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
        _req: &(),
        _client: &ClientWithMiddleware,
        _cache_settings: &CacheServiceSettings,
    ) -> Result<(), AggregatorError> {
        Err(AggregatorError::StoringCacheError(Some("Cache by IP address not supported.".to_string())))
    }
}
