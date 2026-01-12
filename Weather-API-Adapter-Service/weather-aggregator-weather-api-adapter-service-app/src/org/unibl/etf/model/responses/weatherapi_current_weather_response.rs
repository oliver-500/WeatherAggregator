use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WeatherAPICurrentWeatherResponse {
    #[serde(default)]
    pub location: Option<Location>,
    #[serde(default)]
    pub current: Option<Current>,

}

#[derive(Debug, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub lat: Option<f64>, //latitude
    #[serde(default)]
    pub lon: Option<f64>, //longitude
    #[serde(default)]
    pub name: Option<String>, // Location name
    #[serde(default)]
    pub country: Option<String>, //Country name
    #[serde(default)]
    pub region: Option<String>,
}

impl Default for Location {
    fn default() -> Location {
        Location {
            lat: None,
            lon: None,
            name: None,
            country: None,
            region: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Current {
    #[serde(default)]
    pub temp_c: Option<f64>, //temperature in c
    #[serde(default)]
    pub temp_f: Option<f64>, //temperature in f
    #[serde(default)]
    pub feelslike_c: Option<f64>,
    #[serde(default)]
    pub feelslike_f: Option<f64>,
    #[serde(default)]
    pub condition: Option<Condition>,
    #[serde(default)]
    pub pressure_mb: Option<f64>, //pressure in mb
    #[serde(default)]
    pub pressure_in: Option<f64>,
    #[serde(default)]
    pub humidity: Option<f64>, //relative humidity
    #[serde(default)]
    pub wind_mph: Option<f64>, //wind speed in mph
    #[serde(default)]
    pub wind_kph: Option<f64>, //wind speed in kph
    #[serde(default)]
    pub wind_degree: Option<u16>, //wind direction in degrees
    #[serde(default)]
    pub wind_dir: Option<String>,
    #[serde(default)]
    pub gust_mph: Option<f64>, //wind gusts speed in mph
    #[serde(default)]
    pub gust_kph: Option<f64>, //wind gusts speed in kph
    #[serde(default)]
    pub last_updated_epoch: Option<i64>, //last updated
    #[serde(default)]
    pub cloud: Option<f64>, //cloud cover in percentage
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    #[serde(default)]
    pub text: Option<String>, //condition, e.g. rain
    #[serde(default)]
    pub code: Option<u16> //condition code
}

impl Default for Condition {
    fn default() -> Condition {
        Condition {
            text: None,
            code: None
        }
    }
}