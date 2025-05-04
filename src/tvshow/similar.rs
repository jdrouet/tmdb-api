use std::borrow::Cow;

use crate::common::PaginatedResult;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Params<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Cow<'a, str>>,
    /// Specify which page to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl<'a> Params<'a> {
    pub fn set_page(&mut self, value: u32) {
        self.page = Some(value);
    }

    pub fn with_page(mut self, value: u32) -> Self {
        self.set_page(value);
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

impl<E: crate::client::Executor> crate::Client<E> {
    /// Command to get similar tvshows
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_similar_tvshows(1, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_similar_tvshows(
        &self,
        tvshow_id: u64,
        params: &Params<'_>,
    ) -> crate::Result<PaginatedResult<super::TVShowShort>> {
        let url = format!("/tv/{tvshow_id}/similar");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-similar.json"))
            .create_async()
            .await;
        let result = client
            .get_similar_tvshows(1399, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 20);
        assert_eq!(result.total_pages, 2074);
        assert_eq!(result.total_results, 41463);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "Sunset Vibes");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = client
            .get_similar_tvshows(1399, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = client
            .get_similar_tvshows(1399, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let result = client
            .get_similar_tvshows(1399, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }
}
