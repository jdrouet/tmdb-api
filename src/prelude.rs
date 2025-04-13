use std::borrow::Cow;

use crate::client::Executor;

pub trait Command: Sync {
    type Output: serde::de::DeserializeOwned;

    fn path(&self) -> Cow<'static, str>;
    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    fn execute<E: Executor + Send + Sync>(
        &self,
        client: &crate::Client<E>,
    ) -> impl Future<Output = Result<Self::Output, crate::error::Error>> + Send {
        async move { client.execute(self.path().as_ref(), self.params()).await }
    }
}
