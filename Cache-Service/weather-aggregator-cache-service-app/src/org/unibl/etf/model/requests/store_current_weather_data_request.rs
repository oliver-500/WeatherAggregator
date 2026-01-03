use serde::Deserialize;
use validator::Validate;
use crate::org::unibl::etf::model::responses::current_weather_cache_response::CurrentWeatherCacheResponse;

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct StoreCurrentWeatherDataRequest {
    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: f64,
    #[validate(range(min = -180.0, max = 180.0))]
    pub lon: f64,
    pub current_weather_data: CurrentWeatherCacheResponse,
}