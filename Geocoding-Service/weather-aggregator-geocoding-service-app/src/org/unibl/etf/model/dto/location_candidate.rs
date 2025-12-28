use serde::{Serialize};
use crate::org::unibl::etf::model::responses::geocoding_api_response::GeocodingAPIResponse;

#[derive(Debug, Serialize, Clone)]
pub struct LocationCandidate {
    pub location_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

impl TryFrom<GeocodingAPIResponse> for LocationCandidate {
    type Error = String;
    fn try_from(response: GeocodingAPIResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            location_name: response.name, //mozda dodati i alternative names da user ima vise info!!!
            state: response.state.unwrap_or(String::default()),
            country: response.country,
            lat: response.lat,
            lon: response.lon,
        })
    }
}