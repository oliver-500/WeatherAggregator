use serde::{Serialize};
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::util::serializers::serialize_empty_string;

#[derive(Serialize, Debug)]
pub struct DownstreamCurrentWeatherRequest {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    pub limit: u16,

    #[serde(default = "default_provider")]
    #[serde(serialize_with = "serialize_empty_string")]
    pub provider: Option<String>,
}

impl TryFrom<&UpstreamCurrentWeatherRequest> for DownstreamCurrentWeatherRequest {
    type Error = String;

    fn try_from(req: &UpstreamCurrentWeatherRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            lat: req.lat,
            lon: req.lon,
            location: req.location.clone(),
            limit: req.limit,
            provider: Some(default_provider()), // !!!ucitati const iz env
        })
    }
}



fn default_provider() -> String {
    "weatherapi.com".to_owned()
}