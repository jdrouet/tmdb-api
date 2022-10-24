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
