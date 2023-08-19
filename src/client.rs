use reqwest::StatusCode;
use std::borrow::Cow;

const BASE_URL: &str = "https://api.themoviedb.org/3";

/// HTTP client for TMDB
///
/// ```rust
/// use tmdb_api::Client;
///
/// let client = Client::new("this-is-my-secret-token".into());
/// ```
#[cfg(feature = "commands")]
pub struct Client {
    client: reqwest::Client,
    base_url: Cow<'static, str>,
    api_key: String,
}

#[cfg(feature = "commands")]
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    base_url: Cow<'static, str>,
}

#[cfg(feature = "commands")]
impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: Cow::Borrowed(BASE_URL),
            api_key,
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Cow::Owned(base_url);
        self
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
        let res = self.client.get(url).query(&params).send().await?;
        let status_code = res.status();
        if status_code.is_success() {
            Ok(res.json::<T>().await?)
        } else if status_code == StatusCode::UNPROCESSABLE_ENTITY {
            let payload: crate::error::ServerValidationBodyError = res.json().await?;
            Err(crate::error::Error::from((status_code, payload.into())))
        } else {
            let payload: crate::error::ServerOtherBodyError = res.json().await?;
            Err(crate::error::Error::from((status_code, payload.into())))
        }
    }
}

#[cfg(feature = "commands")]
impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            client: None,
            base_url: Cow::Borrowed(BASE_URL),
        }
    }
}

#[cfg(feature = "commands")]
impl ClientBuilder {
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn base_url<T: Into<Cow<'static, str>>>(mut self, base_url: T) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn build(self, api_key: String) -> Client {
        Client {
            client: self.client.unwrap_or_default(),
            base_url: self.base_url,
            api_key,
        }
    }
}
