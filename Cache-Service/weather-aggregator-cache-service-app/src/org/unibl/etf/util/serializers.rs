use chrono::{DateTime, Utc};
use serde::Serializer;

pub fn format_milliseconds<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // This formats the date as a string with 3-digit millisecond precision
    let s = format!("{}", date.format("%Y-%m-%dT%H:%M:%S%.3fZ"));
    serializer.serialize_str(&s)
}