use std::borrow::Cow;
use std::collections::HashMap;

use crate::client::Executor;
use crate::common::ResultsResponse;

use super::WatchProvider;

#[derive(Debug, Default, serde::Serialize)]
pub struct ListWatchProviderParams<'a> {
    /// ISO 3166-1 alpha-2 value to filter the results for one country.
    pub watch_region: Option<Cow<'a, str>>,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<Cow<'a, str>>,
}

impl<'a> ListWatchProviderParams<'a> {
    pub fn set_watch_region(&mut self, value: impl Into<Cow<'a, str>>) {
        self.watch_region = Some(value.into());
    }

    pub fn with_watch_region(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_watch_region(value);
        self
    }

    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WatchProviderDetail {
    /// A hash map of display priority by country code
    pub display_priorities: HashMap<String, u64>,
    #[serde(flatten)]
    pub inner: WatchProvider,
}

impl<E: Executor> crate::Client<E> {
    /// List watch providers for movies
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_movie_watch_providers(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_movie_watch_providers(
        &self,
        params: &ListWatchProviderParams<'_>,
    ) -> crate::Result<Vec<WatchProviderDetail>> {
        self.execute::<ResultsResponse<Vec<WatchProviderDetail>>, _>(
            "/watch/providers/movie",
            params,
        )
        .await
        .map(|res| res.results)
    }

    /// List watch providers for tvshows
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_movie_watch_providers(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_tvshow_watch_providers(
        &self,
        params: &ListWatchProviderParams<'_>,
    ) -> crate::Result<Vec<WatchProviderDetail>> {
        self.execute::<ResultsResponse<Vec<WatchProviderDetail>>, _>("/watch/providers/tv", params)
            .await
            .map(|res| res.results)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/watch/providers/movie")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/watch-provider-movie-list.json"))
            .create_async()
            .await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_movie_watch_providers(&Default::default())
            .await
            .unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/watch-provider-tv-list.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_tvshow_watch_providers(&Default::default())
            .await
            .unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client
            .list_tvshow_watch_providers(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/watch/providers/tv")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client
            .list_tvshow_watch_providers(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::ListWatchProviderParams;
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let params = ListWatchProviderParams::default().with_language("en-US");
        let result = client.list_tvshow_watch_providers(&params).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let params = ListWatchProviderParams::default().with_language("en-US");
        let result = client.list_movie_watch_providers(&params).await.unwrap();
        assert!(!result.is_empty());
    }
}
