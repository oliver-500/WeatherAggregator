use futures::future::join_all;
use reqwest_middleware::ClientWithMiddleware;
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::AdapterServiceError;
use crate::org::unibl::etf::model::requests::downstream_current_weather_request::DownstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::requests::store_current_weatcher_cache_request::StoreCurrentWeatherDataRequest;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::strategy::composite_strategy::CompositeStrategy;
use crate::org::unibl::etf::strategy::priority_strategy::PriorityStrategy;
use crate::org::unibl::etf::strategy::weather_strategy::WeatherStrategy;

#[derive(Debug)]
pub struct CurrentWeatherService {

}


impl CurrentWeatherService {
    fn new() -> Self {
        Self {
        }
    }

    #[tracing::instrument(name = "Get Cached Current Weather Data Function", skip(client, cache_service_settings))]
    pub async fn get_cached_current_weather_data(
        &self,
        req: &DownstreamCurrentWeatherRequest,
        client: &ClientWithMiddleware,
        cache_service_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        let mut params = Vec::new();
        params.push(("lat", req.lat.unwrap().to_string()));
        params.push(("lon", req.lon.unwrap().to_string()));

        let url = format!("{}://{}:{}/api/v1/current_weather", cache_service_settings.scheme, cache_service_settings.host, cache_service_settings.port);

        let response = client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

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

    #[tracing::instrument(name = "Get Current Weather Data Service", skip(client, cache_service_settings, providers_settings))]
    pub async fn get_current_weather_data(
        &self,
        request: &UpstreamCurrentWeatherRequest,
        client: &ClientWithMiddleware,
        providers_settings: &Vec<ProviderSettings>,
        cache_service_settings: &CacheServiceSettings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {

        let req = DownstreamCurrentWeatherRequest::try_from(request)
            .map_err(|e| AggregatorError::ServerError(Some(e.to_string())))?;

        if req.lat.is_some() || req.lon.is_some() {
            match self.get_cached_current_weather_data(
                &req,
                client,
                cache_service_settings,
            ).await {
                Ok(current_weather_data) => {
                    tracing::info!("Cache hit while getting current weather data. {:?}", current_weather_data);
                    return Ok(current_weather_data)
                },
                Err(e) => {
                    tracing::info!("Was not able to get current weather cached data: {}", e.get_message());
                }
            }
        }
        else {
            tracing::info!("Not checking for current weather cached data since latitude and longitude are not provided in the request.")
        }

        let request_futures = providers_settings.iter().map(|provider| {
            let client = client.clone();
            let req = req.clone();

            async move {
                let url = format!("{}://{}:{}/api/v1/current_weather", provider.scheme, provider.host, provider.port);
                let mut params = Vec::new();

                if let Some(lat) = req.lat {
                    params.push(("lat", lat.to_string()));
                }
                if let Some(lon) = req.lon {
                    params.push(("lon", lon.to_string()));
                }
                if let Some(name) = req.location_name {
                    if !name.is_empty() {
                        params.push(("location_name", name));
                    }
                }

                let response = client
                    .get(url)
                    .query(&params)
                    .send()
                    .await
                    .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?; //kasnije ukloniti poruku

                if response.status().is_success() {
                    let body_text = response.text()
                        .await
                        .map_err(|e| AggregatorError::ServerError(
                            Some(format!("Failed to get {} Adapter Service success response body text: {}", provider.name, e))
                        ))?;

                    println!("joj {}", body_text);

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
        });

        let results = join_all(request_futures).await;

        let results: Vec<WeatherProviderResult> =
            results.into_iter().enumerate().map(|(index, res)| {
                let provider = providers_settings[index].name.clone();
                match res {
                    Ok(data) => {
                        tracing::info!("Success from provider: {}, response: {:?}", providers_settings[index].name, data);
                        WeatherProviderResult {
                            provider,
                            data: Some(data),
                            error: None,
                        }
                    },
                    Err(e) => {
                        tracing::error!("Error from provider {}: {}", providers_settings[index].name, e.get_message());
                        WeatherProviderResult {
                            provider,
                            data: None,
                            error: Some(e),
                        }
                    },
                }
            }).collect();

        let errors: Vec<&AggregatorError> = results.iter().filter_map(|r| r.error.as_ref()).collect();
        if let Some(candidates) = errors.iter().find_map(|e| {
            if let AggregatorError::AmbiguousLocationNameError(candidates) = e {
                Some(candidates.clone())
            } else {
                None
            }
        }) {
            return Err(AggregatorError::AmbiguousLocationNameError(candidates));
        }

        if !results.is_empty() && results.iter().all(|e| matches!(e.error, Some(AggregatorError::LocationNotFoundError(_)))) {
            if let Some(AggregatorError::LocationNotFoundError(location)) = errors.first() {
                return Err(AggregatorError::LocationNotFoundError(location.clone()));
            }
        }

        let strategy = CompositeStrategy {
            average_min: 2,
            priority: PriorityStrategy {
                order: vec!["openweathermap.org".into(), "weatherapi.com".into()],
            },
        };
        let final_weather_result = strategy.resolve(&results);

        match final_weather_result {
            None => {
                Err(AggregatorError::WeatherDataUnavailableError)
            }
            Some(res) => {
                let store_request = StoreCurrentWeatherDataRequest {
                    current_weather_data: res.clone(),
                    lat: req.lat.unwrap_or(res.location.lat),
                    lon: req.lon.unwrap_or(res.location.lon)
                };

                match self.save_current_weather_data_to_cache(
                    client,
                    cache_service_settings,
                    &store_request
                ).await {
                    Ok(_) => {
                        tracing::info!("Successfully saved current weather data to cache.");
                    },
                    Err(e) => {
                        tracing::error!("Error saving current weather data to cache with error: {}", e.get_message());
                    }
                }
                Ok(res)
            }
        }
    }

    #[tracing::instrument(name = "Send Current Weather Data to Cache Service function", skip(client, cache_service_settings))]
    pub async fn save_current_weather_data_to_cache(
        &self,
        client: &ClientWithMiddleware,
        cache_service_settings: &CacheServiceSettings,
        data: &StoreCurrentWeatherDataRequest,
    ) -> Result<bool, AggregatorError> {
        let url = format!("{}://{}:{}/api/v1/current_weather", cache_service_settings.scheme, cache_service_settings.host, cache_service_settings.port);

        let response = client
            .put(url)
            .json(&data)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?;

        if response.status().is_success() {
            Ok(true)
        } else {
            let error_body_text = response.text().await.map_err(|e| {
                AggregatorError::ServerError(Some(format!("Failed to get error body text: {}", e)))
            })?;

            let error_body: CacheServiceError = serde_json::from_str(&error_body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
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


impl Default for CurrentWeatherService {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug)]
pub struct WeatherProviderResult {
    pub provider: String,
    pub data: Option<CurrentWeatherResponse>,
    pub error: Option<AggregatorError>,
}