
use crate::org::unibl::etf::util::serializers::round_serialize;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_f64;
use crate::org::unibl::etf::util::serializers::serialize_empty_string;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_u8;
use crate::org::unibl::etf::util::serializers::serialize_empty_i64;
use crate::org::unibl::etf::util::serializers::serialize_empty_u16;

use serde::{Serialize};
use crate::org::unibl::etf::model::errors::adapter_service_error::AdapterServiceError;
use crate::org::unibl::etf::model::responses::weatherapi_current_weather_response::{WeatherAPICurrentWeatherResponse, Location as WeatherAPI_Location, Condition};


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


impl TryFrom<WeatherAPICurrentWeatherResponse> for UniformCurrentWeatherResponse {
    type Error = AdapterServiceError;

    fn try_from(src: WeatherAPICurrentWeatherResponse) -> Result<Self, Self::Error> {
        let current = src.current.ok_or(
            AdapterServiceError::InvalidProviderResponseError(Some("Missing current field".to_string()))
        )?;
        let location = src.location.unwrap_or(WeatherAPI_Location::default());
        let condition = current.condition.unwrap_or(Condition::default());
        let temp_c = current.temp_c.ok_or(
            AdapterServiceError::InvalidProviderResponseError(Some("Missing mandatory temperature field".to_string()))
        )?;
        let temp_f = current.temp_f.ok_or(
            AdapterServiceError::InvalidProviderResponseError(Some("Missing mandatory temperature field".to_string()))
        )?;

        Ok(UniformCurrentWeatherResponse {
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


