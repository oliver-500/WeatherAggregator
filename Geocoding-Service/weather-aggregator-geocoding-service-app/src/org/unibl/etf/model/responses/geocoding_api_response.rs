use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GeocodingAPIResponse {
    pub name: String,
    local_names: Option<HashMap<String, String>>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: Option<String>,
}