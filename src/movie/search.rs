use std::borrow::Cow;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Params<'a> {
    /// ISO 639-1 value to display translated data for the fields that support
    /// it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Cow<'a, str>>,
    /// Which page to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Whether to include adult (pornography) content in the results.
    #[serde(skip_serializing_if = "crate::util::is_false")]
    pub include_adult: bool,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_release_year: Option<u16>,
}

impl<'a> Params<'a> {
    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }

    pub fn set_page(&mut self, value: u32) {
        self.page = Some(value);
    }

    pub fn with_page(mut self, value: u32) -> Self {
        self.set_page(value);
        self
    }

    pub fn set_include_adult(&mut self, value: bool) {
        self.include_adult = value;
    }

    pub fn with_include_adult(mut self, value: bool) -> Self {
        self.set_include_adult(value);
        self
    }

    pub fn set_region(&mut self, value: impl Into<Cow<'a, str>>) {
        self.region = Some(value.into());
    }

    pub fn with_region(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_region(value);
        self
    }

    pub fn set_year(&mut self, value: u16) {
        self.year = Some(value);
    }

    pub fn with_year(mut self, value: u16) -> Self {
        self.set_year(value);
        self
    }

    pub fn set_primary_release_year(&mut self, value: u16) {
        self.primary_release_year = Some(value);
    }

    pub fn with_primary_release_year(mut self, value: u16) -> Self {
        self.set_primary_release_year(value);
        self
    }
}

#[derive(serde::Serialize)]
struct WithQuery<'a, V> {
    query: Cow<'a, str>,
    #[serde(flatten)]
    inner: V,
}

impl<E: crate::client::Executor> crate::Client<E> {
    /// Search for movies by their original, translated and alternative titles.
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.search_movies("die hard", &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn search_movies<'a>(
        &self,
        query: impl Into<Cow<'a, str>>,
        params: &Params<'a>,
    ) -> crate::Result<crate::common::PaginatedResult<super::MovieShort>> {
        self.execute(
            "/search/movie",
            &WithQuery {
                query: query.into(),
                inner: params,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/search/movie")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/search-movie.json"))
            .create_async()
            .await;
        let result = client
            .search_movies("Whatever", &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
        assert!(result.total_pages > 0);
        assert!(result.total_results > 0);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.title, "RRRrrrr!!!");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/search/movie")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = client
            .search_movies("Whatever", &Default::default())
            .await
            .unwrap_err();
        println!("err {err:?}");
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/search/movie")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = client
            .search_movies("Whatever", &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }

    #[tokio::test]
    async fn validation_error() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/search/movie")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "".into()),
            ]))
            .with_status(422)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/validation-error.json"))
            .create_async()
            .await;
        let err = client
            .search_movies("", &Default::default())
            .await
            .unwrap_err();
        let validation_err = err.as_validation_error().unwrap();
        assert_eq!(validation_err.errors.len(), 1);
    }

    // #[tokio::test]
    // async fn premature_end_of_line() {
    // let mut server = mockito::Server::new_async().await;
    // let client =
    // Client::<ReqwestClient>::builder().with_api_key("secret".into()).
    // with_base_url(server.url()).build().unwrap();

    //     let client =
    // Client::<ReqwestClient>::new("secret".into()).
    // with_base_url(mockito::server_url());     let cmd =
    // MovieSearch::new("game of thrones".into());

    //     let _m = mock("GET", super::PATH)
    //         .match_query(Matcher::AllOf(vec![
    //             Matcher::UrlEncoded("api_key".into(), "secret".into()),
    //             Matcher::UrlEncoded("query".into(), "game of
    // thrones".into()),         ]))
    //         .with_status(200)
    //         .with_header("content-type", "application/json;charset=utf-8")
    //         .with_body(include_str!("../../assets/search-tv-decoding-error.
    // json"))         .create_async().await;
    //     let result = cmd.execute(&client).await.unwrap();
    //     assert_eq!(result.page, 1);
    // }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn search_rrrrrrr() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let result = client
            .search_movies("Rrrrrrr", &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 1);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.title, "RRRrrrr!!!");
    }

    #[tokio::test]
    async fn search_simpsons() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let _result = client
            .search_movies("simpsons", &Default::default())
            .await
            .unwrap();
    }
}
