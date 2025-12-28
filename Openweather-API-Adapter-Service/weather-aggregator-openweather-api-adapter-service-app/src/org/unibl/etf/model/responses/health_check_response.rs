use chrono::{DateTime, Utc};
use serde::{Serialize};
use crate::org::unibl::etf::util::serializers::format_milliseconds;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub(crate) status: String,
    pub(crate) service_name: String,
    #[serde(serialize_with = "format_milliseconds")]
    pub(crate) timestamp: DateTime<Utc>,
}

