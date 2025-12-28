use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct CurrentWeatherRequestRaw {
    pub location_name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Deserialize, Validate)]
#[serde(try_from = "CurrentWeatherRequestRaw")]
pub struct CurrentWeatherRequest {
    pub location_name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,

}

impl TryFrom<CurrentWeatherRequestRaw> for CurrentWeatherRequest {
    type Error = String;

    fn try_from(raw: CurrentWeatherRequestRaw) -> Result<Self, Self::Error> {
        if (raw.location_name.is_none() || raw.location_name.clone().unwrap().is_empty())
            && (raw.lat.is_none() || raw.lon.is_none()) {
            return Err(String::from("Location name or coordinates (latitude and longitude) should be provided"));
        }

        let lat_validated = match raw.lat {
            Some(lat) => {
                (-90.0..=90.0).contains(&lat)
            },
            None => {
                true
            },
        };
        let lon_validated = match raw.lon {
            Some(lon) => {
                (-180.0..=180.0).contains(&lon)
            },
            None => {
                true
            },
        };

        if lon_validated && lat_validated {
            return Ok(CurrentWeatherRequest {
                lat: raw.lat,
                lon: raw.lon,
                location_name: raw.location_name,
            });
        } else if !lat_validated {
            return Err(format!("Invalid latitude: {}", raw.lat.ok_or("")?));
        } else {
            return Err(format!("Invalid longitude: {}", raw.lon.ok_or("")?));
        }

    }
}
