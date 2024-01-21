//! Module to help deserializing dates that could be empty

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

#[allow(dead_code)]
pub(crate) fn serialize<S>(value: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = value {
        let s = super::date::format_date(date);
        serializer.serialize_str(s.as_str())
    } else {
        serializer.serialize_str("")
    }
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        None => Ok(None),
        Some(inner) if inner.is_empty() => Ok(None),
        Some(inner) => super::date::parse_date(&inner)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestingStruct {
        #[serde(deserialize_with = "crate::util::empty_date::deserialize")]
        value: Option<NaiveDate>,
    }

    #[test]
    fn should_deserialize() {
        let result: TestingStruct = serde_json::from_str(r#"{"value":""}"#).unwrap();
        assert_eq!(result.value, None);
    }
}
