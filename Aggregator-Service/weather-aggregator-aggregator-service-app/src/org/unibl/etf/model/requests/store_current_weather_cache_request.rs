use serde::Serialize;
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;


#[derive(Debug, Serialize, Clone)]
pub struct StoreCurrentWeatherCacheRequest{
    pub lat: f64,
    pub lon: f64,
    pub current_weather_data: CurrentWeatherResponse,
    pub location_names: Vec<String>
}