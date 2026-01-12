use std::net::IpAddr;
use serde::{Serialize};

use crate::org::unibl::etf::model::requests::upstream_current_weather_request_by_coordinates::UpstreamCurrentWeatherRequestByCoordinates;
use crate::org::unibl::etf::model::requests::upstream_current_weather_request_by_location::UpstreamCurrentWeatherRequestByLocation;


#[derive(Debug, Clone, Serialize)]
pub struct DownstreamCurrentWeatherRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>

}

impl TryFrom<&UpstreamCurrentWeatherRequestByLocation> for DownstreamCurrentWeatherRequest {
    type Error = String;

    fn try_from(req: &UpstreamCurrentWeatherRequestByLocation) -> Result<Self, Self::Error> {
        Ok(Self {
            lat: None,
            lon: None,
            location_name: Some(req.location_name.clone()),
            ip_address: None,
        })
    }
}

impl TryFrom<&UpstreamCurrentWeatherRequestByCoordinates> for DownstreamCurrentWeatherRequest {
    type Error = String;

    fn try_from(req: &UpstreamCurrentWeatherRequestByCoordinates) -> Result<Self, Self::Error> {
        Ok(Self {
            lat: Some(req.lat),
            lon: Some(req.lon),
            location_name: None,
            ip_address: None,
        })
    }
}

impl TryFrom<&IpAddr> for DownstreamCurrentWeatherRequest {
    type Error = String;

    fn try_from(req: &IpAddr) -> Result<Self, Self::Error> {
        Ok(Self {
            lat: None,
            lon: None,
            location_name: None,
            ip_address: Some(req.to_string()),
        })
    }
}