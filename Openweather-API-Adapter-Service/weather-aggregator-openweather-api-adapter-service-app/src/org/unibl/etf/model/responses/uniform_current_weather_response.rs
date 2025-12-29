
use crate::org::unibl::etf::util::serializers::round_serialize;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_f64;
use crate::org::unibl::etf::util::serializers::serialize_empty_string;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_u8;
use crate::org::unibl::etf::util::serializers::serialize_empty_i64;
use crate::org::unibl::etf::util::serializers::serialize_empty_u16;

use serde::{Serialize};
use crate::org::unibl::etf::model::errors::adapter_service_error::AdapterServiceError;
use crate::org::unibl::etf::model::responses::openweather_current_weather_response::{Coordinates, OpenWeatherAPICurrentWeatherResponse, Sys, Wind as Wind_OpenWeather};
use crate::org::unibl::etf::util::convertors::{celsius_to_fahrenheit, degrees_to_cardinal, kph_to_mph, mb_to_inhg};

#[derive(Debug, Serialize)]
pub struct UniformCurrentWeatherResponse {
    pub provider: String,
    pub location: Location,
    pub weather: Weather,
    pub wind: Wind,
    #[serde(serialize_with = "serialize_empty_i64")]
    pub observed_at_timestamp: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Location {
    #[serde(serialize_with = "serialize_empty_string")]
    pub name: Option<String>,
    #[serde(serialize_with = "serialize_empty_string")]
    pub country: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct Wind {
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub speed_metric: Option<f64>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub speed_imperial: Option<f64>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub gust_metric: Option<f64>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub gust_imperial: Option<f64>,
    #[serde(serialize_with = "serialize_empty_string")]
    pub direction: Option<String>,
    #[serde(serialize_with = "serialize_empty_u16")]
    pub degrees: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct Weather {
    #[serde(serialize_with = "round_serialize")]
    pub temp_metric: f64,
    #[serde(serialize_with = "round_serialize")]
    pub temp_imperial: f64,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub temp_feelslike_metric: Option<f64>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub temp_feelslike_imperial: Option<f64>,

    #[serde(serialize_with = "serialize_and_round_empty_u8")]
    pub humidity: Option<u8>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub pressure_metric: Option<f64>,
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub pressure_imperial: Option<f64>,
    #[serde(serialize_with = "serialize_empty_string")]
    pub condition: Option<String>,
}

impl TryFrom<OpenWeatherAPICurrentWeatherResponse> for UniformCurrentWeatherResponse {
    type Error = AdapterServiceError;

    fn try_from(src: OpenWeatherAPICurrentWeatherResponse) -> Result<Self, Self::Error> {
        let weather = src.weather
            .get(0)
            .ok_or(AdapterServiceError::InvalidProviderResponseError(Some("Empty weather array field found".to_string())))?;

        let main = src.main.ok_or(AdapterServiceError::InvalidProviderResponseError(Some("Missing mandatory value. Empty main field found".to_string())))?;
        let coordinates = src.coord.unwrap_or(Coordinates::default());
        let sys = src.sys.unwrap_or(Sys::default());
        let wind = src.wind.unwrap_or(Wind_OpenWeather::default());
        let temp = main.temp.ok_or(AdapterServiceError::InvalidProviderResponseError(Some("Missing mandatory value. Empty main:temp field found".to_string())))?;

        Ok(UniformCurrentWeatherResponse {
            provider: "openweathermap.org".into(), //env!!!
            location: Location {
                name: src.name,
                country: sys.country,
                lat: coordinates.lat,
                lon: coordinates.lon,
            },
            weather: Weather {
                temp_metric: temp,
                temp_imperial: celsius_to_fahrenheit(temp),
                temp_feelslike_metric: main.feels_like,
                temp_feelslike_imperial: main.feels_like.map(|fl| celsius_to_fahrenheit(fl)),
                humidity: Some(main.humidity.unwrap_or(f64::default()) as u8),
                pressure_metric: main.pressure,
                pressure_imperial: main.pressure.map(|mb| mb_to_inhg(mb)),
                condition: weather.description.clone(),
            },
            wind: Wind {
                speed_metric: wind.speed,
                speed_imperial: wind.speed.map(|s| kph_to_mph(s)),
                gust_metric: wind.gust,
                gust_imperial: wind.gust.map(|g| kph_to_mph(g)),
                direction: wind.deg.map(|d| degrees_to_cardinal(d).to_string()),
                degrees: wind.deg.map(|d| d as u16),
            },
            observed_at_timestamp: src.dt,

        })
    }
}




