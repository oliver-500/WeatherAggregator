use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_location_presence"))]
pub struct UpstreamCurrentWeatherRequest {
    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: Option<f64>,

    #[validate(range(min = -180.0, max = 180.0))]
    pub lon: Option<f64>,

    pub location_name: Option<String>,

    #[validate(range(min = 1, max = 1000))]
    #[serde(default = "default_limit")]
    pub limit: u16,
}

fn default_limit() -> u16 {
    5
}

fn validate_location_presence(params: &UpstreamCurrentWeatherRequest) -> Result<(), ValidationError> {
    // Logic: If location_name is missing, then BOTH lat and lon must be present.
    if params.location_name.is_none() {
        if params.lat.is_none() || params.lon.is_none() {
            let mut error = ValidationError::new("incomplete_location");
            error.message = Some("Must provide either a location name OR both lat/lon coordinates.".into());
            return Err(error);
        }
    }
    Ok(())
}