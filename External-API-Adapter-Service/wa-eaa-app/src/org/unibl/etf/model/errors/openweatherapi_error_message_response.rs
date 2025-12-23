use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenWeatherAPIErrorMessageResponse {
    pub message: String,
    pub cod: u16,
}