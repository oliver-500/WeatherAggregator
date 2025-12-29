use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_service::WeatherProviderResult;

pub trait WeatherStrategy {
    fn resolve(&self, results: &[WeatherProviderResult]) -> Option<CurrentWeatherResponse>;
}