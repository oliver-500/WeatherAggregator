use crate::org::unibl::etf::model::responses::current_weather_response::{CurrentWeatherResponse, Location, Weather, Wind};
use crate::org::unibl::etf::services::current_weather_service::WeatherProviderResult;
use crate::org::unibl::etf::strategy::weather_strategy::WeatherStrategy;

pub struct AverageStrategy;

impl WeatherStrategy for AverageStrategy {
    fn resolve(&self, results: &[WeatherProviderResult]) -> Option<CurrentWeatherResponse> {
        let valid: Vec<&CurrentWeatherResponse> =
            results.iter().filter_map(|r| r.data.as_ref()).collect();

        if valid.is_empty() {
            return None;
        }

        Some(CurrentWeatherResponse {
            provider: "aggregated".to_string(),

            location: Location {
                name: valid.iter().find_map(|r| r.location.name.clone()),
                country: valid.iter().find_map(|r| r.location.country.clone()),
                lat: avg_opt_f64(valid.iter().map(|r| Some(r.location.lat))).unwrap_or(0.0),
                lon: avg_opt_f64(valid.iter().map(|r| Some(r.location.lon))).unwrap_or(0.0),
            },

            weather: Weather {
                temp_metric: avg_f64(valid.iter().map(|r| r.weather.temp_metric))?,
                temp_imperial: avg_f64(valid.iter().map(|r| r.weather.temp_imperial))?,

                temp_feelslike_metric: avg_opt_f64(
                    valid.iter().map(|r| r.weather.temp_feelslike_metric),
                ),
                temp_feelslike_imperial: avg_opt_f64(
                    valid.iter().map(|r| r.weather.temp_feelslike_imperial),
                ),

                humidity: avg_opt_u8(
                    valid.iter().map(|r| r.weather.humidity),
                ),

                pressure_metric: avg_opt_f64(
                    valid.iter().map(|r| r.weather.pressure_metric),
                ),
                pressure_imperial: avg_opt_f64(
                    valid.iter().map(|r| r.weather.pressure_imperial),
                ),

                condition: valid.iter().find_map(|r| r.weather.condition.clone()),
            },

            wind: Wind {
                speed_metric: avg_opt_f64(
                    valid.iter().map(|r| r.wind.speed_metric),
                ),
                speed_imperial: avg_opt_f64(
                    valid.iter().map(|r| r.wind.speed_imperial),
                ),
                gust_metric: avg_opt_f64(
                    valid.iter().map(|r| r.wind.gust_metric),
                ),
                gust_imperial: avg_opt_f64(
                    valid.iter().map(|r| r.wind.gust_imperial),
                ),

                direction: valid.iter().find_map(|r| r.wind.direction.clone()),
                degrees: avg_opt_u16(
                    valid.iter().map(|r| r.wind.degrees),
                ),
            },

            observed_at_timestamp: avg_opt_f64(
                valid.iter().map(|r| r.observed_at_timestamp.map(|v| v as f64)),
            ).map(|v| v as i64),
        })
    }
}

fn avg_f64(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, count) = values.fold((0.0, 0), |(s, c), v| (s + v, c + 1));
    if count > 0 { Some(sum / count as f64) } else { None }
}

fn avg_opt_f64(values: impl Iterator<Item = Option<f64>>) -> Option<f64> {
    avg_f64(values.filter_map(|v| v))
}

fn avg_opt_u8(values: impl Iterator<Item = Option<u8>>) -> Option<u8> {
    avg_f64(values.filter_map(|v| v.map(|x| x as f64)))
        .map(|v| v.round() as u8)
}

fn avg_opt_u16(values: impl Iterator<Item = Option<u16>>) -> Option<u16> {
    avg_f64(values.filter_map(|v| v.map(|x| x as f64)))
        .map(|v| v.round() as u16)
}