use serde::{Deserialize, };
use crate::org::unibl::etf::model::responses::current_weather_cache_response::CurrentWeatherCacheResponse;

#[derive(Debug, Deserialize, Clone)]
pub struct MultipleCachedLocationsResponse {
    pub candidates: Vec<CurrentWeatherCacheResponse>,
}
