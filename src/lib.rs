/// The used version of reqwest
pub use reqwest;

pub mod company;
pub mod error;
pub mod movie;
pub mod prelude;
pub mod tvshow;

mod common;
mod util;

use std::borrow::Cow;

const BASE_URL: &str = "https://api.themoviedb.org/3";

pub struct Client {
    client: reqwest::Client,
    base_url: Cow<'static, str>,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: Cow::Borrowed(BASE_URL),
            api_key,
        }
    }

    #[cfg(test)]
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Cow::Owned(base_url);
        self
    }

    async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        mut params: Vec<(&str, Cow<'_, str>)>,
    ) -> Result<T, error::Error> {
        params.push(("api_key", Cow::Borrowed(self.api_key.as_str())));

        let url = format!("{}{}", self.base_url, path);
        let res = self.client.get(url).query(&params).send().await?;
        let status_code = res.status();
        if status_code.is_success() {
            let payload: T = res.json().await?;
            Ok(payload)
        } else {
            let payload: error::ServerBodyError = res.json().await?;
            Err(error::Error::from((status_code, payload)))
        }
    }
}
