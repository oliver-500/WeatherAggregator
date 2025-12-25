use serde::Serializer;

pub fn serialize_empty_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
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


pub fn serialize_empty_u16<S>(value: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_u16(*v),
        None => serializer.serialize_str(""), // Frontend gets ""
    }
}

pub fn round_serialize<S>(val: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Round to 1 decimal place
    let rounded = (val * 10.0).round() / 10.0;
    s.serialize_f64(rounded)
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