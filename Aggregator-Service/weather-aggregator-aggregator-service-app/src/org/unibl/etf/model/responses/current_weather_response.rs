use serde::{Deserialize, Serialize};
use crate::org::unibl::etf::util::deserializers::{
    deserialize_i64_or_empty_string_as_none,
    deserialize_u8_or_empty_string_as_none,
    deserialize_u16_or_empty_string_as_none,
    deserialize_f64_or_empty_string_as_none
};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CurrentWeatherResponse {
    pub provider: String,
    pub location: Location,
    pub weather: Weather,
    pub wind: Wind,

    #[serde(deserialize_with = "deserialize_i64_or_empty_string_as_none")]
    pub observed_at_timestamp: Option<i64>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Location {

    pub name: Option<String>,
    pub country: Option<String>,

    // #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    // pub lat: Option<f64>,
    pub lat: f64,

    // #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    // pub lon: Option<f64>,
    pub lon: f64
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Wind {
    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub speed_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub speed_imperial: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub gust_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub gust_imperial: Option<f64>,


    pub direction: Option<String>,

    #[serde(deserialize_with = "deserialize_u16_or_empty_string_as_none")]
    pub degrees: Option<u16>,
}



#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Weather {

    pub temp_metric: f64,

    pub temp_imperial: f64,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub temp_feelslike_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub temp_feelslike_imperial: Option<f64>,

    #[serde(deserialize_with= "deserialize_u8_or_empty_string_as_none")]
    pub humidity: Option<u8>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub pressure_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    pub pressure_imperial: Option<f64>,

    pub condition: Option<String>,
}