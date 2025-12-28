use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::configuration;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::ExternalAPIAdapterError;
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
        config: &configuration::Settings,
    ) -> Result<CurrentWeatherResponse, AggregatorError> {

        //provjeriti cache
        //ako se desi cache miss slati zahtjeve na oba endpointa za temp
        let req = DownstreamCurrentWeatherRequest::try_from(request).map_err(|_e| AggregatorError::ServerError(None))?;

        let response = client
            .post(format!("http://{}:{}/api/v1/currentweather", config.external_api_adapter.host, config.external_api_adapter.port))
            .json(&req)
            .send()
            .await
            .map_err(|e| AggregatorError::ConnectionError(e.to_string()))?;

        if response.status().is_success() {
            let body_text = response.text()
                .await
                .map_err(|e| AggregatorError::ServerError(
                    Some(format!("Failed to get success body text: {}", e))
                ))?;

            let res: CurrentWeatherResponse = serde_json::from_str(&body_text)
                .map_err(|e| {
                    AggregatorError::ResponseParsingError(format!(
                        "Failed to parse response: JSON Error: {} | Raw Body: {}",
                        e, body_text
                    ))
                })?;

            return Ok(res);
        } else {
            let error_body_text = response.text().await.map_err(|e| {
                AggregatorError::ServerError(Some(format!("Failed to get error body text: {}", e)))
            })?;

            let error_body: ExternalAPIAdapterError = serde_json::from_str(&error_body_text)
                .map_err(|e| {
                    // This 'e' will now contain the EXACT field and line number
                    AggregatorError::ResponseParsingError(format!(
                        "JSON Error: {} | Raw Body: {}",
                        e, error_body_text
                    ))
                })?;
            println!("Response: {:?}",  error_body);

            return Err(AggregatorError::from(error_body.error.code));
        }
    }
}


impl Default for CurrentWeatherService {
    fn default() -> Self {
        Self::new()
    }
}