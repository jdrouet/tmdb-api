//! Module to help deserializing strings that could be empty

use serde::{Deserialize, Deserializer, Serializer};

#[allow(dead_code)]
pub(crate) fn serialize<S>(value: Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_some(&value)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(|value| value.filter(|inner| !inner.is_empty()))
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Serialize, Deserialize)]
    struct TestingStruct {
        #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
        value: Option<String>,
    }

    #[test]
    fn should_deserialize() {
        let result: TestingStruct = serde_json::from_str(r#"{"value":""}"#).unwrap();
        assert_eq!(result.value, None);
    }
}
