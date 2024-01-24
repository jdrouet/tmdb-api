use std::borrow::Cow;
#[cfg(feature = "tokio-rate-limit")]
use std::{ops::Sub, time::Duration};

use reqwest::StatusCode;
#[cfg(feature = "tokio-rate-limit")]
use tokio::{
    sync::RwLock,
    time::{sleep, Instant},
};

const BASE_URL: &str = "https://api.themoviedb.org/3";
#[cfg(feature = "tokio-rate-limit")]
const REQUESTS_PER_SECOND: u64 = 50;

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
    /// The tmdb api has a rate limit of 50 requests per second per api key for 20 ip addresses.
    /// It may be useful if the api key is shared between multiple applications to have a precise
    /// control over the number of requests per second for each application.
    #[cfg(feature = "tokio-rate-limit")]
    requests_per_second: Option<u64>,
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

    #[cfg(feature = "tokio-rate-limit")]
    pub fn with_requests_per_second(mut self, value: u64) -> Self {
        self.requests_per_second = Some(value);
        self
    }

    #[cfg(feature = "tokio-rate-limit")]
    pub fn set_requests_per_second(mut self, value: u64) {
        self.requests_per_second = Some(value);
    }

    pub fn build(self) -> Result<Client, ClientBuilderError> {
        let base_url = self.base_url;
        let client = self.client.unwrap_or_default();
        let api_key = self.api_key.ok_or(ClientBuilderError::MissingApiKey)?;
        #[cfg(feature = "tokio-rate-limit")]
        let requests_per_second = self.requests_per_second.unwrap_or(REQUESTS_PER_SECOND);
        #[cfg(feature = "tokio-rate-limit")]
        let request_interval = Duration::from_micros(1_000_000 / requests_per_second);

        Ok(Client {
            client,
            base_url,
            api_key,
	        #[cfg(feature = "tokio-rate-limit")]
            // Subtract the request interval to ensure that the first request is sent immediately.
	        start_timestamp: Instant::now().sub(request_interval),
	        #[cfg(feature = "tokio-rate-limit")]
	        last_request_timestamp_ms: RwLock::new(0),
	        #[cfg(feature = "tokio-rate-limit")]
	        request_interval_ms: request_interval.as_millis() as u64,
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
    #[cfg(feature = "tokio-rate-limit")]
    /// The timestamp of reference for the rate limit.
    start_timestamp: Instant,
    #[cfg(feature = "tokio-rate-limit")]
    /// The timestamp at which the last request was sent.
    last_request_timestamp_ms: RwLock<u64>,
    #[cfg(feature = "tokio-rate-limit")]
    request_interval_ms: u64,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn new(api_key: String) -> Self {
        #[cfg(feature = "tokio-rate-limit")]
        let request_interval = Duration::from_micros(1_000_000 / REQUESTS_PER_SECOND);

        Self {
            client: reqwest::Client::default(),
            base_url: Cow::Borrowed(BASE_URL),
            api_key,
	        #[cfg(feature = "tokio-rate-limit")]
            // Subtract the request interval to ensure that the first request is sent immediately.
            start_timestamp: Instant::now().sub(request_interval),
	        #[cfg(feature = "tokio-rate-limit")]
            last_request_timestamp_ms: RwLock::new(0),
	        #[cfg(feature = "tokio-rate-limit")]
            request_interval_ms: request_interval.as_millis() as u64,
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
        #[cfg(feature = "tokio-rate-limit")]
        {
            // Ensure that the order of the requests is respected.
            let mut last_request_timestamp_ms = self.last_request_timestamp_ms.write().await;

            let now_ms = Instant::now()
                .duration_since(self.start_timestamp)
                .as_millis() as u64;
            let elapsed_ms = now_ms - *last_request_timestamp_ms;

            if elapsed_ms < self.request_interval_ms {
                sleep(Duration::from_millis(self.request_interval_ms - elapsed_ms)).await;
            }

            *last_request_timestamp_ms = Instant::now()
                .duration_since(self.start_timestamp)
                .as_millis() as u64;
        }

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
