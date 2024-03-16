use std::borrow::Cow;
use std::collections::HashMap;

use crate::common::MediaType;

use super::WatchProvider;

/// Command to get the details of a collection
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::watch_provider::list::WatchProviderList;
///
/// #[tokio::main]
/// async fn main() {
///     use tmdb_api::common::MediaType;
/// let client = Client::new("this-is-my-secret-token".into());
///     let cmd = WatchProviderList::new(MediaType::Tv);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug)]
pub struct WatchProviderList {
    pub media_type: MediaType,
    /// ISO 3166-1 alpha-2 value to filter the results for one country.
    pub watch_region: Option<String>,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl WatchProviderList {
    pub fn new(media_type: MediaType) -> Self {
        Self {
            media_type,
            watch_region: None,
            language: None,
        }
    }

    pub fn with_watch_region(mut self, watch_region: String) -> Self {
        self.watch_region = Some(watch_region);
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WatchProviderListResult {
    /// A hash map of display priority by country code
    pub display_priorities: HashMap<String, u64>,
    #[serde(flatten)]
    pub inner: WatchProvider,
}

#[async_trait::async_trait]
impl crate::prelude::Command for WatchProviderList {
    type Output = Vec<WatchProviderListResult>;

    fn path(&self) -> Cow<'static, str> {
        format!("/watch/providers/{}", self.media_type).into()
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut params = Vec::new();

        if let Some(watch_region) = self.watch_region.as_ref() {
            params.push(("watch_region", Cow::Borrowed(watch_region.as_str())));
        }
        if let Some(language) = self.language.as_ref() {
            params.push(("language", Cow::Borrowed(language.as_str())));
        }
        params
    }

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        #[derive(Deserialize)]
        struct Result {
            pub results: Vec<WatchProviderListResult>,
        }

        client
            .execute::<Result>(self.path().as_ref(), self.params())
            .await
            .map(|res| res.results)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::common::MediaType;
    use crate::prelude::Command;
    use crate::Client;

    use super::WatchProviderList;

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = WatchProviderList::new(MediaType::Movie);

        let _m = server
            .mock("GET", "/watch/providers/movie")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/watch-provider-movie-list.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = WatchProviderList::new(MediaType::Tv);

        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/watch-provider-tv-list.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = WatchProviderList::new(MediaType::Tv);

        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = WatchProviderList::new(MediaType::Tv);

        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::common::MediaType;
    use crate::prelude::Command;
    use crate::Client;

    use super::WatchProviderList;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let mut cmd = WatchProviderList::new(MediaType::Tv);
        cmd.language = Some("en-US".into());

        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let mut cmd = WatchProviderList::new(MediaType::Movie);
        cmd.language = Some("en-US".into());

        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
