use serde::Deserialize;
use validator::Validate;



#[derive(Deserialize, Validate, Debug)]
#[serde(try_from = "RetrieveCurrentWeatherCacheRequestRaw")]
pub struct RetrieveCurrentWeatherCacheRequest {

    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    pub lat: Option<f64>,

    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    pub lon: Option<f64>,

    pub location_name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,

}



#[derive(Deserialize)]
pub struct RetrieveCurrentWeatherCacheRequestRaw {
    pub location_name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,
}

impl TryFrom<RetrieveCurrentWeatherCacheRequestRaw> for RetrieveCurrentWeatherCacheRequest {
    type Error = String;

    fn try_from(raw: RetrieveCurrentWeatherCacheRequestRaw) -> Result<Self, Self::Error> {
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
            return Ok(RetrieveCurrentWeatherCacheRequest {
                lat: raw.lat,
                lon: raw.lon,
                location_name: raw.location_name,
                country: raw.country,
                state: raw.state
            });
        } else if !lat_validated {
            return Err(format!("Invalid latitude: {}", raw.lat.ok_or("")?));
        } else {
            return Err(format!("Invalid longitude: {}", raw.lon.ok_or("")?));
        }

    }
}