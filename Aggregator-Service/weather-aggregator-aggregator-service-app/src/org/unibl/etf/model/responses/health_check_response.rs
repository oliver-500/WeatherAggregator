use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub(crate) status: String,
    pub(crate) service_name: String,
    pub(crate) timestamp: DateTime<Utc>,
}