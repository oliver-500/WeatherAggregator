use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpenWeatherAPICurrentWeatherResponse {
    #[serde(default)]
    pub weather: Vec<Weather>,
    #[serde(default)]
    pub main: Option<Main>,
    #[serde(default)]
    pub name: Option<String>, // Location name
    #[serde(default)]
    pub coord: Option<Coordinates>,
    #[serde(default)]
    pub wind: Option<Wind>,
    #[serde(default)]
    pub dt: Option<i64>, //measurement timestamp in unix epoch format(UTC)
    #[serde(default)]
    pub sys: Option<Sys>,
    #[serde(default)]
    pub timezone: Option<i32>, //shift in seconds from UTC
    #[serde(default)]
    pub clouds: Option<Clouds>
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    #[serde(default)]
    pub main: Option<String>, //condition, e.g. rain
    #[serde(default)]
    pub description: Option<String>, //condition description, e.g. rain
    #[serde(default)]
    pub id: Option<u16> //condition code
}

#[derive(Debug, Deserialize)]
pub struct Main {
    #[serde(default)]
    pub temp: Option<f64>, //temperature in f or c
    #[serde(default)]
    pub feels_like: Option<f64>,
    #[serde(default)]
    pub pressure: Option<f64>,  //pressure in mb
    #[serde(default)]
    pub humidity: Option<f64>, //relative humidity
}


#[derive(Debug, Deserialize)]
pub struct Coordinates {
    #[serde(default)]
    pub lat: Option<f64>, //latitude
    #[serde(default)]
    pub lon: Option<f64>, //longitude
}

impl Default for Coordinates {
    fn default() -> Self {
        Coordinates { lat: None, lon: None }
    }
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    #[serde(default)]
    pub speed: Option<f64>, //wind speed in either kph or mph
    #[serde(default)]
    pub gust: Option<f64>,  //wind gusts speed in either kph or mph
    #[serde(default)]
    pub deg: Option<f64>, //wind direction in degrees
}

impl Default for Wind {
    fn default() -> Self {
        Wind { speed: None, gust: None, deg: None, }
    }
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    #[serde(default)]
    pub country: Option<String>, //country name code
}

impl Default for Sys {
    fn default() -> Self {
        Sys {
            country: Some("".to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    #[serde(default)]
    pub all: Option<f64>, //cloud cover in percentage
}