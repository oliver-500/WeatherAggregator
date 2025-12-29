use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_service::WeatherProviderResult;
use crate::org::unibl::etf::strategy::weather_strategy::WeatherStrategy;

pub struct PriorityStrategy {
    pub order: Vec<String>,
}

impl WeatherStrategy for PriorityStrategy {
    fn resolve(&self, results: &[WeatherProviderResult]) -> Option<CurrentWeatherResponse> {
        for provider in &self.order {
            if let Some(r) = results.iter().find(|r| &r.provider == provider) {
                if let Some(data) = &r.data {
                    return Some(data.clone());
                }
            }
        }
        None
    }
}