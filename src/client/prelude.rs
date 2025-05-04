pub trait Executor: Default + Send + Sync {
    fn execute<T: serde::de::DeserializeOwned, P: serde::Serialize>(
        &self,
        url: &str,
        params: P,
    ) -> impl Future<Output = crate::Result<T>>;
}
