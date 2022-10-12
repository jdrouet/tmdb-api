use std::borrow::Cow;

#[async_trait::async_trait]
pub trait Command {
    type Output: serde::de::DeserializeOwned;

    fn path(&self) -> Cow<'static, str>;
    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client.execute(self.path().as_ref(), self.params()).await
    }
}
