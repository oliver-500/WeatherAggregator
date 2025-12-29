
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_service::WeatherProviderResult;
use crate::org::unibl::etf::strategy::average_strategy::AverageStrategy;
use crate::org::unibl::etf::strategy::priority_strategy::PriorityStrategy;
use crate::org::unibl::etf::strategy::weather_strategy::WeatherStrategy;

pub struct CompositeStrategy {
    pub average_min: usize,
    pub priority: PriorityStrategy,
}

impl WeatherStrategy for CompositeStrategy {
    fn resolve(&self, results: &[WeatherProviderResult]) -> Option<CurrentWeatherResponse> {
        let successes: Vec<&CurrentWeatherResponse> =
            results.iter().filter_map(|r| r.data.as_ref()).collect();

        if successes.len() >= self.average_min {
            return AverageStrategy.resolve(results);
        }

        if let Some(prioritized) = self.priority.resolve(results) {
            return Some(prioritized);
        }

        // FINAL fallback: any successful provider
        successes.first().cloned().cloned()
    }
}