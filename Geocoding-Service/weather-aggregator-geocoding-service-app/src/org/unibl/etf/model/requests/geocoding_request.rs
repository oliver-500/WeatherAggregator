use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct GeocodingRequest {
    #[validate(length(min = 1, max = 200, message = "Location name length should be between 1 and 200 characters."))]
    pub location_name: String,
    #[validate(range(min = 0, max = 5, message = "Maximum of 5 results allowed in geocoding (limit query parameter)."))]
    #[serde(default = "default_limit")]
    pub limit: Option<u16>,
}

pub fn default_limit() -> Option<u16> {
    Some(5)
}