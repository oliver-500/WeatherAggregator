use futures::future::join_all;
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::configuration::settings::ProviderSettings;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::AdapterServiceError;
use crate::org::unibl::etf::model::requests::downstream_current_weather_request::DownstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::strategy::composite_strategy::CompositeStrategy;
use crate::org::unibl::etf::strategy::priority_strategy::PriorityStrategy;
use crate::org::unibl::etf::strategy::weather_strategy::WeatherStrategy;

pub struct CurrentWeatherService {

}

impl CurrentWeatherService {
    fn new() -> Self {
        Self {
        }
    }

    pub async fn get_current_weather_data(
        &self,
        request: &UpstreamCurrentWeatherRequest,
        client: &reqwest::Client,
        providers_settings: &Vec<ProviderSettings>,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {

        let req = DownstreamCurrentWeatherRequest::try_from(request).map_err(|_e| AggregatorError::ServerError(None))?;

        //provjeriti cache
        //ako se desi cache miss slati zahtjeve na oba endpointa za temp

        let request_futures = providers_settings.iter().map(|provider| {
            let client = client.clone(); // reqwest::Client is an Arc internally, so cloning is cheap
            let req = req.clone();

            println!("op");
            // We move the async block into the collection
            async move {
                let url = format!("http://{}:{}/api/v1/current_weather", provider.host, provider.port);

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

                println!("op2");
                let response = client
                    .get(url)
                    .query(&params)
                    .send()
                    .await
                    .map_err(|e| AggregatorError::ConnectionError(Some(e.to_string())))?; //kasnije ukloniti poruku

                // Note: You probably want to deserialize here too
                if response.status().is_success() {
                    let body_text = response.text()
                        .await
                        .map_err(|e| AggregatorError::ServerError(
                            Some(format!("Failed to get success body text: {}", e))
                        ))?;

                    println!("op5");
                    let res: CurrentWeatherResponse = serde_json::from_str(&body_text)
                        .map_err(|e| {
                            AggregatorError::ResponseParsingError(Some(format!(
                                "Failed to parse response: JSON Error: {} | Raw Body: {}",
                                e, body_text
                            )))
                        })?;

                    return Ok(res);
                } else {
                    let error_body_text = response.text().await.map_err(|e| {
                        AggregatorError::ServerError(Some(format!("Failed to get error body text: {}", e)))
                    })?;

                    let error_body: AdapterServiceError = serde_json::from_str(&error_body_text)
                        .map_err(|e| {
                            // This 'e' will now contain the EXACT field and line number
                            AggregatorError::ResponseParsingError(Some(format!(
                                "JSON Error: {} | Raw Body: {}",
                                e, error_body_text
                            )))
                        })?;
                    println!("Response: {:?}",  error_body);

                    Err(AggregatorError::from(error_body.error.code))
                }
            }
        });

        // 2. Execute all futures concurrently
        // join_all returns a Vec<Result<...>> in the same order as the providers
        let results = join_all(request_futures).await;


        // 3. Process the results

        let results: Vec<WeatherProviderResult> =
            results.into_iter().enumerate().map(|(index, res)| {
                let provider = providers_settings[index].name.clone();
                match res {
                    Ok(data) => {
                        println!("Success from provider: {}, response: {:?}", providers_settings[index].name, data);
                        WeatherProviderResult {
                            provider,
                            data: Some(data),
                            error: None,
                        }
                    },
                    Err(e) => {
                        println!("Error from provider {}: {}", providers_settings[index].name, e.get_message());
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

        if let Some(location) = errors.iter().find_map(|e| {
            if let AggregatorError::LocationNotFoundError(location) = e {
                Some(location.clone())
            } else {
                None
            }
        }) {
            return Err(AggregatorError::LocationNotFoundError(location));
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
                Ok(res)
            }
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