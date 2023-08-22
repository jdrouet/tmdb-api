use reqwest::StatusCode;
use std::borrow::Cow;

const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Debug)]
pub enum ClientBuilderError {
    MissingApiKey,
}

impl std::fmt::Display for ClientBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing api key")
    }
}

impl std::error::Error for ClientBuilderError {}

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Cow<'static, str>,
    client: Option<reqwest::Client>,
    api_key: Option<String>,
}

impl ClientBuilder {
    pub fn with_base_url<U: Into<Cow<'static, str>>>(mut self, value: U) -> Self {
        self.base_url = value.into();
        self
    }

    pub fn set_base_url<U: Into<Cow<'static, str>>>(&mut self, value: U) {
        self.base_url = value.into();
    }

    pub fn with_reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn set_reqwest_client(mut self, client: reqwest::Client) {
        self.client = Some(client);
    }

    pub fn with_api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn set_api_key(mut self, value: String) {
        self.api_key = Some(value);
    }

    pub fn build(self) -> Result<Client, ClientBuilderError> {
        let base_url = self.base_url;
        let client = self.client.unwrap_or_default();
        let api_key = self.api_key.ok_or(ClientBuilderError::MissingApiKey)?;

        Ok(Client {
            client,
            base_url,
            api_key,
        })
    }
}

/// HTTP client for TMDB
///
/// ```rust
/// use tmdb_api::Client;
///
/// let client = Client::new("this-is-my-secret-token".into());
/// ```
pub struct Client {
    client: reqwest::Client,
    base_url: Cow<'static, str>,
    api_key: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::default(),
            base_url: Cow::Borrowed(BASE_URL),
            api_key,
        }
    }

    #[deprecated = "Use client builder instead. This will get dropped in future versions."]
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
