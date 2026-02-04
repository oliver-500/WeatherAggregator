use actix_web::web;
use futures::future::join_all;
use reqwest_middleware::ClientWithMiddleware;
use tracing::Instrument;
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::configuration::settings::{CacheServiceSettings, ProviderSettings};
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::LocationCandidate;
use crate::org::unibl::etf::model::requests::current_weather_query::CurrentWeatherQuery;

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

    #[tracing::instrument(name = "Get Current Weather Data Service", skip(client, cache_service_settings, providers_settings))]
    pub async fn get_current_weather<Q: CurrentWeatherQuery + std::fmt::Debug + Clone + 'static>(
        &self,
        query: Q,
        client: web::Data<ClientWithMiddleware>,
        providers_settings: web::Data<Vec<ProviderSettings>>,
        cache_service_settings: web::Data<CacheServiceSettings>,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {
        println!("op2");
        let get_cache_req = query.build_retrieve_cache_request();

        println!("op3");
        let cache_candidates: Vec<CurrentWeatherResponse> = match get_cache_req {
            Ok(get_cache_req) => {
                match query.cache_get(&get_cache_req, client.as_ref(), cache_service_settings.as_ref()).await {
                    Ok(cached_data) => {
                        tracing::info!("Cache hit for current weather data. {:?}", cached_data);
                        return Ok(cached_data);
                    },
                    Err(error) => {
                        match error {
                            AggregatorError::OnlyPotentialMatchesFoundError(candidates) => {
                                candidates
                            },
                            _ => {
                                tracing::error!("Was not able to get cached current weather data.");
                                Vec::new()
                            }
                        }
                    }
                }
            },
            Err(e) => {
                match e {
                    AggregatorError::CacheNotSupported(_) => {
                        tracing::error!("Cache not supported. {:?}", e);
                        println!("op4");
                        Vec::new()
                    },
                    e => {
                        return Err(e);
                    }
                }
            }
        };


        println!("op5");
        let req = query.build_downstream_request()?;
        let futures = providers_settings.iter().map(|provider| {
            let client = client.clone();
            let req = req.clone();
            let query = query.clone();

            async move {
                query.call_provider(provider, &req, &client).await
            }
        });
        let results = join_all(futures).await;

        let normalized = results
            .into_iter()
            .enumerate()
            .map(|(i, res)| {
                let provider = providers_settings[i].name.clone();
                match res {
                    Ok(data) => WeatherProviderResult {
                        provider,
                        data: Some(data),
                        error: None,
                    },
                    Err(e) => WeatherProviderResult {
                        provider,
                        data: None,
                        error: Some(e),
                    },
                }
            })
            .collect::<Vec<_>>();

        let errors: Vec<&AggregatorError> = normalized.iter().filter_map(|r| r.error.as_ref()).collect();
        if let Some(mut candidates) = errors.iter().find_map(|e| {
            if let AggregatorError::AmbiguousLocationNameError(candidates) = e {
                Some(candidates.clone())
            } else {
                None
            }
        }) {
            let mut additional_candidates: Vec<LocationCandidate> = Vec::new();
            for cache_candidate in cache_candidates {
                additional_candidates.push(
                    LocationCandidate {
                        location_name: cache_candidate.location.name.unwrap(),
                        state: cache_candidate.location.state_region_province_or_entity.unwrap(),
                        country: cache_candidate.location.country.unwrap(),
                        lat: cache_candidate.location.lat,
                        lon: cache_candidate.location.lon,
                    }
                );
            }
            candidates.extend(additional_candidates);
            return Err(AggregatorError::AmbiguousLocationNameError(candidates));
        }

        // Error precedence logic (unchanged)
        if let Some(candidates) = normalized.iter().find_map(|r| {
            if let Some(AggregatorError::AmbiguousLocationNameError(c)) = &r.error {
                Some(c.clone())
            } else {
                None
            }
        }) {
            return Err(AggregatorError::AmbiguousLocationNameError(candidates));
        }

        if normalized.iter().all(|r| matches!(r.error, Some(AggregatorError::LocationNotFoundError(_)))) {
            if let Some(AggregatorError::LocationNotFoundError(loc)) =
                normalized.iter().filter_map(|r| r.error.as_ref()).next()
            {
                return Err(AggregatorError::LocationNotFoundError(loc.clone()));
            }
        }

        let strategy = CompositeStrategy {
            average_min: 2,
            priority: PriorityStrategy {
                order: vec!["openweathermap.org".into(), "weatherapi.com".into()],
            },
        };
        let result = strategy
            .resolve(&normalized)
            .ok_or(AggregatorError::WeatherDataUnavailableError)?;


        let query_clone = query.clone();
        let result_clone = result.clone();
        let cache_service_settings_clone = cache_service_settings.clone();
        let client_clone = client.clone();


        actix_web::rt::spawn(async move {
            let mut location_names: Vec<String> = normalized
                .iter()
                .filter_map(|res| {
                    res.data.as_ref().and_then(|d| d.location.name.clone())
                })
                .collect();

            location_names.sort();
            location_names.dedup();

            let store_cache_request = query_clone.build_store_cache_request(&result_clone, location_names);
            match store_cache_request {
                Ok(store_cache_request) => {
                    let _ = query_clone
                        .cache_set(&store_cache_request, &client_clone, cache_service_settings_clone.as_ref())
                        .await;
                },
                Err(e) => {
                    tracing::error!("Storing cache error: {:?}", e);
                }
            }
        }.instrument(tracing::Span::current()));

        Ok(result)
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