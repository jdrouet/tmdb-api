use std::borrow::Cow;

pub trait Executor: Default + Send + Sync {
    fn execute<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        params: Vec<(&str, Cow<'_, str>)>,
    ) -> impl Future<Output = Result<T, crate::error::Error>> + Send;
}
