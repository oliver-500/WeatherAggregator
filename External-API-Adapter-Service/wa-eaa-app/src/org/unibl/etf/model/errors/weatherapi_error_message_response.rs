use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherAPIErrorMessageResponse {
    pub error: WeatherAPIError,

}



#[derive(Debug, Deserialize)]
pub struct WeatherAPIError {
    pub message: String,
    pub code: u16,
}