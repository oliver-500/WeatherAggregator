
use crate::org::unibl::etf::util::serializers::round_serialize;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_f64;
use crate::org::unibl::etf::util::serializers::serialize_empty_string;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_u8;
use crate::org::unibl::etf::util::serializers::serialize_empty_i64;
use crate::org::unibl::etf::util::serializers::serialize_empty_u16;

use serde::{Serialize, Serializer};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;
use crate::org::unibl::etf::model::responses::openweatherapi_current_weather_response::{Coordinates, OpenWeatherAPICurrentWeatherResponse, Sys, Wind as Wind_OpenWeather};
use crate::org::unibl::etf::model::responses::weatherapi_current_weather_response::{WeatherAPICurrentWeatherResponse, Location as WeatherAPI_Location, Condition};



#[derive(Debug, Serialize)]
pub struct UnifiedCurrentWeatherResponse {
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

impl TryFrom<OpenWeatherAPICurrentWeatherResponse> for UnifiedCurrentWeatherResponse {
    type Error = AdapterError;

    fn try_from(src: OpenWeatherAPICurrentWeatherResponse) -> Result<Self, Self::Error> {
        let weather = src.weather
            .get(0)
            .ok_or(AdapterError::InvalidProviderResponse("Empty weather array field found".to_string()))?;

        let main = src.main.ok_or(AdapterError::InvalidProviderResponse("Missing mandatory value. Empty main field found".to_string()))?;
        let coordinates = src.coord.unwrap_or(Coordinates::default());
        let sys = src.sys.unwrap_or(Sys::default());
        let wind = src.wind.unwrap_or(Wind_OpenWeather::default());
        let temp = main.temp.ok_or(AdapterError::InvalidProviderResponse("Missing mandatory value. Empty main:temp field found".to_string()))?;

        Ok(UnifiedCurrentWeatherResponse {
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


//staviti u util
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn mb_to_inhg(mb: f64) -> f64 {
    // 1 mb = 0.0295299830714 inHg
    mb * 0.0295299830714
}

fn kph_to_mph(kph: f64) -> f64 {
    // 1 km = 0.62137119 miles
    kph * 0.62137119
}

fn degrees_to_cardinal(degrees: f64) -> &'static str {
    // 1. Normalize degrees to 0.0 - 360.0
    let degrees = degrees % 360.0;

    // 2. Define the 16 directions
    let directions = [
        "N", "NNE", "NE", "ENE",
        "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW",
        "W", "WNW", "NW", "NNW"
    ];

    // 3. Each segment is 22.5 degrees wide (360 / 16)
    // We add 11.25 (half a segment) so that "N" covers the range 348.75 to 11.25
    let index = ((degrees + 11.25) / 22.5) as usize;

    // 4. Use modulo 16 to wrap the 360/0 boundary back to "N"
    directions[index % 16]
}


impl TryFrom<WeatherAPICurrentWeatherResponse> for UnifiedCurrentWeatherResponse {
    type Error = AdapterError;

    fn try_from(src: WeatherAPICurrentWeatherResponse) -> Result<Self, Self::Error> {
        let current = src.current.ok_or(AdapterError::InvalidProviderResponse("Missing current field".to_string()))?;
        let location = src.location.unwrap_or(WeatherAPI_Location::default());
        let condition = current.condition.unwrap_or(Condition::default());
        let temp_c = current.temp_c.ok_or(AdapterError::InvalidProviderResponse("Missing mandatory temperature field".to_string()))?;
        let temp_f = current.temp_f.ok_or(AdapterError::InvalidProviderResponse("Missing mandatory temperature field".to_string()))?;

        Ok(UnifiedCurrentWeatherResponse {
            provider: "weatherapi.com".to_string(),
            location: Location {
                name: location.name,
                country: location.country,
                lat: location.lat,
                lon: location.lon,
            },
            weather: Weather {
                temp_metric: temp_c,
                temp_imperial: temp_f,
                temp_feelslike_metric: current.feelslike_c,
                temp_feelslike_imperial: current.feelslike_f,
                humidity: current.humidity.map(|h| h as u8),
                pressure_metric: current.pressure_mb,
                pressure_imperial: current.pressure_in,
                condition: condition.text,//doraditi jer se moze razlikovati kod openweathera i weathera
            },
            wind: Wind {
                speed_metric: current.wind_kph,
                speed_imperial: current.wind_mph,
                gust_metric: current.gust_kph,
                gust_imperial: current.gust_mph,
                direction: current.wind_dir,
                degrees: current.wind_degree,
            },
            observed_at_timestamp: current.last_updated_epoch,
        })
    }
}