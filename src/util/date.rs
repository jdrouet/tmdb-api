//! Module to help serializing and deserializing tmdb dates

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d";

pub(super) fn parse_date(input: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(input, FORMAT)
}

pub(crate) fn format_date(input: &NaiveDate) -> String {
    input.format(FORMAT).to_string()
}

#[allow(dead_code)]
pub(crate) fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(FORMAT).to_string();
    serializer.serialize_str(&s)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_date(&value).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct TestingStruct {
        #[serde(with = "crate::util::date")]
        value: chrono::NaiveDate,
    }

    #[test]
    fn should_serialize() {
        let value = TestingStruct {
            value: chrono::NaiveDate::from_ymd_opt(1990, 1, 22).unwrap(),
        };
        let result = serde_json::to_string(&value).unwrap();
        assert_eq!(result, r#"{"value":"1990-01-22"}"#);
    }

    #[test]
    fn should_deserialize() {
        let date = chrono::NaiveDate::from_ymd_opt(1990, 1, 22).unwrap();
        let result: TestingStruct = serde_json::from_str(r#"{"value":"1990-01-22"}"#).unwrap();
        assert_eq!(result.value, date);
    }
}
