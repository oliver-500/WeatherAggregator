use serde::Serialize;
use crate::org::unibl::etf::model::dto::location_candidate::LocationCandidate;

#[derive(Debug, Serialize)]
pub struct GeocodingResponse {
    pub(crate) candidates: Vec<LocationCandidate>
}