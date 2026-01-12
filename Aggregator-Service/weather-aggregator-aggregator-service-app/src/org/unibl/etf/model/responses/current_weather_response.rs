use crate::org::unibl::etf::util::serializers::serialize_empty_i64;
use crate::org::unibl::etf::util::serializers::serialize_empty_string;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_u8;
use crate::org::unibl::etf::util::serializers::serialize_and_round_empty_f64;
use crate::org::unibl::etf::util::deserializers::deserialize_f64_or_empty_string_as_nonee;
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
    #[serde(serialize_with = "serialize_empty_i64")]
    #[serde(deserialize_with = "deserialize_i64_or_empty_string_as_none")]
    pub observed_at_timestamp: Option<i64>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Location {
    #[serde(serialize_with = "serialize_empty_string")]
    pub name: Option<String>,
    #[serde(serialize_with = "serialize_empty_string")]
    pub country: Option<String>,
    pub lat: f64,
    pub lon: f64,
    #[serde(serialize_with = "serialize_empty_string")]
    pub state_region_province_or_entity: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Wind {
    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub speed_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub speed_imperial: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_nonee")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub gust_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub gust_imperial: Option<f64>,

    #[serde(serialize_with = "serialize_empty_string")]
    pub direction: Option<String>,

    #[serde(deserialize_with = "deserialize_u16_or_empty_string_as_none")]
    pub degrees: Option<u16>,
}



#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Weather {
    pub temp_metric: f64,
    pub temp_imperial: f64,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub temp_feelslike_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub temp_feelslike_imperial: Option<f64>,

    #[serde(deserialize_with= "deserialize_u8_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_u8")]
    pub humidity: Option<u8>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub pressure_metric: Option<f64>,

    #[serde(deserialize_with = "deserialize_f64_or_empty_string_as_none")]
    #[serde(serialize_with = "serialize_and_round_empty_f64")]
    pub pressure_imperial: Option<f64>,

    #[serde(serialize_with = "serialize_empty_string")]
    pub condition: Option<String>,
}