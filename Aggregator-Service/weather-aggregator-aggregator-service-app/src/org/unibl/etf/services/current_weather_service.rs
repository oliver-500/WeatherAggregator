use futures::future::join_all;
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::configuration::settings::ProviderSettings;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::AdapterServiceError;
use crate::org::unibl::etf::model::requests::downstream_current_weather_request::DownstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;

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
        for (index, res) in results.into_iter().enumerate() {
            match res {
                Ok(resp) => {
                    println!("Success from provider: {}, response: {:?}", providers_settings[index].name, resp);
                },
                Err(e) => println!("Error from provider {}: {}", providers_settings[index].name, e.get_message()),
            }
        }

        Err(AggregatorError::ServerError(None))

    }
}


impl Default for CurrentWeatherService {
    fn default() -> Self {
        Self::new()
    }
}