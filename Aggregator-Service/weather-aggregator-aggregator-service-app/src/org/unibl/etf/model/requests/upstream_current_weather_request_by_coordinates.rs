use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Debug, Deserialize, Validate, Clone)]
pub struct UpstreamCurrentWeatherRequestByCoordinates {
    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: f64,

    #[validate(range(min = -180.0, max = 180.0))]
    pub lon: f64,
}

