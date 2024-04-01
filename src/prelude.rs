use std::borrow::Cow;

use crate::client::Executor;

#[async_trait::async_trait]
pub trait Command {
    type Output: serde::de::DeserializeOwned;

    fn path(&self) -> Cow<'static, str>;
    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    async fn execute<E: Executor + Send + Sync>(
        &self,
        client: &crate::Client<E>,
    ) -> Result<Self::Output, crate::error::Error> {
        client.execute(self.path().as_ref(), self.params()).await
    }
}
