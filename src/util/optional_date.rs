//! Module to help serializing and deserializing tmdb dates

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

#[allow(dead_code)]
pub(crate) fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(inner) = date.as_ref() {
        super::date::serialize(inner, serializer)
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
        if inner.is_empty() {
            Ok(None)
        } else {
            super::date::parse_date(inner)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    } else {
        Ok(None)
    }
}
