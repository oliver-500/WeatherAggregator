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


pub fn serialize_empty_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_str(""), // Frontend gets ""
    }
}

pub fn serialize_and_round_empty_f64<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{

    match value {
        Some(v) => {
            let rounded = (v * 10.0).round() / 10.0;

            serializer.serialize_f64(rounded)
        },
        None => serializer.serialize_str(""), // Frontend gets ""
    }
}

pub fn serialize_and_round_empty_u8<S>(value: &Option<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{

    match value {
        Some(v) => {
            serializer.serialize_u8(*v)
        },
        None => serializer.serialize_str(""), // Frontend gets ""
    }
}

pub fn serialize_empty_i64<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_i64(*v),
        None => serializer.serialize_str(""), // Frontend gets ""
    }
}

