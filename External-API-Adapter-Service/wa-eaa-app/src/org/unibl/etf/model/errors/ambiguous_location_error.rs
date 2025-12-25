use crate::org::unibl::etf::util::serializers::serialize_empty_string;
use serde::{Serialize, Serializer};
use crate::org::unibl::etf::model::responses::geocoding_response::GeocodingResponse;

#[derive(Serialize, Debug)]
pub struct AmbiguousLocationError {
    pub code: u16,
    pub message: String,
    pub candidates: Vec<Candidate>
}

#[derive(Serialize, Debug, Clone)]
pub struct Candidate {
    pub location: String,
    #[serde(serialize_with = "serialize_empty_string")]
    pub state: Option<String>,
    pub country: String,
    pub lat: f64,
    pub lon: f64
}



impl TryFrom<GeocodingResponse> for Candidate {
    type Error = String;
    fn try_from(response: GeocodingResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            location: response.name, //mozda dodati i alternative names da user ima vise info!
            state: response.state,
            country: response.country,
            lat: response.lat,
            lon: response.lon,
        })
    }
}