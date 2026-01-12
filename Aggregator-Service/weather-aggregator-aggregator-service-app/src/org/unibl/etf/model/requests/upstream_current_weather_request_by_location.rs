use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Debug, Deserialize, Clone, Validate)]
pub struct UpstreamCurrentWeatherRequestByLocation {
    #[validate(length(min = 2, message = "Location name must be at least 2 characters long"))]
    pub location_name: String,

    #[validate(length(min = 2, message = "Country name must be at least 2 characters long"))]
    pub country: Option<String>,

    #[validate(length(min = 2, message = "State name must be at least 2 characters long"))]
    pub state: Option<String>,
}

