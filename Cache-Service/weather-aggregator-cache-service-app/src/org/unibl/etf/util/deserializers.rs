use serde::{Deserialize, Deserializer};

pub fn deserialize_f64_or_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Inner enum to handle the two possible JSON types for this field
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeFloat {
        Float(f64),
        String(String),
    }

    match MaybeFloat::deserialize(deserializer)? {
        MaybeFloat::Float(f) => Ok(Some(f)),
        MaybeFloat::String(s) if s.is_empty() => Ok(None),
        MaybeFloat::String(s) => s.parse::<f64>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

pub fn deserialize_f64_or_empty_string_as_nonee<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Inner enum to handle the two possible JSON types for this field
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeFloat {
        Float(f64),
        String(String),
        Unknown(serde_json::Value),
    }

    match MaybeFloat::deserialize(deserializer)? {
        MaybeFloat::Float(f) => {
            println!("joj1");
            Ok(Some(f))
        },
        MaybeFloat::String(s) if s.is_empty() => {
            println!("joj2");
            Ok(None)
        },
        MaybeFloat::String(s) => {
            println!("joj3");
            s.parse::<f64>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        },
        MaybeFloat::Unknown(value) => {
            println!("joj {}", value);
            Ok(None)
        },
    }
}


pub fn deserialize_u16_or_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    // Inner enum to handle the two possible JSON types for this field
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeFloat {
        Unsigned(u16),
        String(String),
    }

    match MaybeFloat::deserialize(deserializer)? {
        MaybeFloat::Unsigned(f) => Ok(Some(f)),
        MaybeFloat::String(s) if s.is_empty() => Ok(None),
        MaybeFloat::String(s) => s.parse::<u16>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

pub fn deserialize_u8_or_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    // Inner enum to handle the two possible JSON types for this field
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeFloat {
        Unsigned(u8),
        String(String),
    }

    match MaybeFloat::deserialize(deserializer)? {
        MaybeFloat::Unsigned(f) => Ok(Some(f)),
        MaybeFloat::String(s) if s.is_empty() => Ok(None),
        MaybeFloat::String(s) => s.parse::<u8>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

pub fn deserialize_i64_or_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    // Inner enum to handle the two possible JSON types for this field
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeFloat {
        Signed(i64),
        String(String),
    }

    match MaybeFloat::deserialize(deserializer)? {
        MaybeFloat::Signed(f) => Ok(Some(f)),
        MaybeFloat::String(s) if s.is_empty() => Ok(None),
        MaybeFloat::String(s) => s.parse::<i64>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}