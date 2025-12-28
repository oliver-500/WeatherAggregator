use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenWeatherAPIError {
    pub message: String,
    pub cod: u16,
}