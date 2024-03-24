// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error("couldn't execute request")]
//     Request {
//         #[source]
//         source: Box<dyn std::error::Error + Send>,
//     },
//     #[error("couldn't read response")]
//     Response {
//         #[source]
//         source: Box<dyn std::error::Error + Send>,
//     },
//     #[error(transparent)]
//     Validation(crate::error::ServerValidationBodyError),
//     #[error("internal server error with code {code}")]
//     Server {
//         code: u16,
//         #[source]
//         content: crate::error::ServerOtherBodyError,
//     },
// }

use std::borrow::Cow;

#[async_trait::async_trait]
pub trait Executor: Default + Send + Sync {
    async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        params: Vec<(&str, Cow<'_, str>)>,
    ) -> Result<T, crate::error::Error>;
}
