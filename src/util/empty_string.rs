/// Module to help deserializing strings that could be empty
use serde::{self, Deserialize, Deserializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|value| if value.is_empty() { None } else { Some(value) })
}

#[cfg(test)]
mod tests {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
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
