use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Clone)]
pub struct GeocodingResponse {
    pub candidates: Vec<LocationCandidate>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationCandidate {
    pub location_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub state: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}