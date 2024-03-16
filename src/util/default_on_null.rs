//! Deserializes null as the default value for the type.

use serde::{Deserialize, Deserializer, Serializer};

#[allow(dead_code)]
pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: serde::Serialize,
{
    T::serialize(value, serializer)
}

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestingStruct<T>
        where
            T: for<'a> serde::Deserialize<'a> + serde::Serialize + Default,
    {
        #[serde(with = "super")]
        value: T,
    }


    #[test]
    fn should_deserialize() {
        let result: TestingStruct<i32> = serde_json::from_str(r#"{"value":null}"#).unwrap();
        assert_eq!(result.value, 0);

        let result: TestingStruct<i32> = serde_json::from_str(r#"{"value":0}"#).unwrap();
        assert_eq!(result.value, 0);

        let result: TestingStruct<i32> =
            serde_json::from_str(r#"{"value":10}"#).unwrap();
        assert_eq!(result.value, 10);

        let result: TestingStruct<String> =
            serde_json::from_str(r#"{"value":null}"#).unwrap();
        assert_eq!(result.value, "");

        let result: TestingStruct<Vec<i32>> =
            serde_json::from_str(r#"{"value":null}"#).unwrap();
        assert_eq!(result.value, Vec::<i32>::new());
    }

    #[test]
    fn should_serialize() {
        let result = serde_json::to_string(&TestingStruct::<i32> { value: 0 }).unwrap();
        assert_eq!(result, r#"{"value":0}"#);

        let result = serde_json::to_string(&TestingStruct::<i32> {
            value: 10,
        })
            .unwrap();
        assert_eq!(result, r#"{"value":10}"#);

        let result = serde_json::to_string(&TestingStruct::<String> {
            value: "hello".to_string(),
        })
            .unwrap();
        assert_eq!(result, r#"{"value":"hello"}"#);

        let result = serde_json::to_string(&TestingStruct::<Vec<i32>> {
            value: vec![1, 2, 3],
        })
            .unwrap();
        assert_eq!(result, r#"{"value":[1,2,3]}"#);
    }
}
