use std::borrow::Cow;

#[async_trait::async_trait]
pub trait Command {
    type Output: serde::de::DeserializeOwned;

    fn path() -> &'static str;
    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client.execute(Self::path(), self.params()).await
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}
