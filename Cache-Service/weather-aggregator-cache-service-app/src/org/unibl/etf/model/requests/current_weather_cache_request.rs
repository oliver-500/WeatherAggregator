use serde::Deserialize;
use validator::Validate;



#[derive(Deserialize, Validate, Debug)]
pub struct CurrentWeatherCacheRequest {
    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    pub lat: f64,

    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    pub lon: f64,

}
