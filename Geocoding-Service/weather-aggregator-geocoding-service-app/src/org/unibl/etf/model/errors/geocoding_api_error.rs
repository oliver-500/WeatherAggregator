use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExternalGeocodingApiError {
    pub message: String,
    pub cod: u16,
}