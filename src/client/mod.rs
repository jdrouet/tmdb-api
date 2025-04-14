pub mod prelude;
pub mod reqwest;

use std::borrow::Cow;

pub use self::prelude::Executor;
pub type ReqwestClient = Client<reqwest::ReqwestExecutor>;

const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Debug, thiserror::Error)]
pub enum ClientBuilderError {
    #[error("missing api key")]
    MissingApiKey,
}

pub struct ClientBuilder<E: prelude::Executor> {
    base_url: Cow<'static, str>,
    executor: Option<E>,
    api_key: Option<String>,
}

impl<E: prelude::Executor> Default for ClientBuilder<E> {
    fn default() -> Self {
        Self {
            base_url: Cow::Borrowed(BASE_URL),
            executor: None,
            api_key: None,
        }
    }
}

impl<E: prelude::Executor> ClientBuilder<E> {
    pub fn with_base_url<U: Into<Cow<'static, str>>>(mut self, value: U) -> Self {
        self.base_url = value.into();
        self
    }

    pub fn set_base_url<U: Into<Cow<'static, str>>>(&mut self, value: U) {
        self.base_url = value.into();
    }

    pub fn with_executor(mut self, executor: E) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn set_executor(mut self, executor: E) {
        self.executor = Some(executor);
    }

    pub fn with_api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn set_api_key(mut self, value: String) {
        self.api_key = Some(value);
    }

    pub fn build(self) -> Result<Client<E>, ClientBuilderError> {
        let base_url = self.base_url;
        let executor = self.executor.unwrap_or_default();
        let api_key = self.api_key.ok_or(ClientBuilderError::MissingApiKey)?;

        Ok(Client {
            executor,
            base_url,
            api_key,
        })
    }
}

/// HTTP client for TMDB
///
/// ```rust
/// use tmdb_api::client::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
///
/// let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
/// ```
pub struct Client<E> {
    executor: E,
    base_url: Cow<'static, str>,
    api_key: String,
}

impl<E: std::fmt::Debug> std::fmt::Debug for Client<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Client))
            .field("executor", &self.executor)
            .field("base_url", &self.base_url)
            .field("api_key", &"REDACTED")
            .finish()
    }
}

impl<E: Executor> Client<E> {
    pub fn builder() -> ClientBuilder<E> {
        ClientBuilder::default()
    }

    pub fn new(api_key: String) -> Self {
        Self {
            executor: E::default(),
            base_url: Cow::Borrowed(BASE_URL),
            api_key,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        mut params: Vec<(&str, Cow<'_, str>)>,
    ) -> Result<T, crate::error::Error> {
        params.push(("api_key", Cow::Borrowed(self.api_key.as_str())));

        let url = format!("{}{}", self.base_url, path);
        self.executor.execute(&url, params).await
    }
}
