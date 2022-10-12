/// Module to help serializing and deserializing tmdb dates
use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &'static str = "%Y-%m-%d";

#[allow(dead_code)]
pub(crate) fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(inner) = date.as_ref() {
        let s = inner.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    if let Some(inner) = value.as_ref() {
        NaiveDate::parse_from_str(&inner, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
