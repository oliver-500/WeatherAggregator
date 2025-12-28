use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherAPIError {
    pub error: WeatherAPIErrorDetails,
}

#[derive(Debug, Deserialize)]
pub struct WeatherAPIErrorDetails {
    pub message: String,
    pub code: u16,
}