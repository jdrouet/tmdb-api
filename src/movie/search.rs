use std::borrow::Cow;

const PATH: &str = "/search/movie";

/// Command to search for movies
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::client::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::movie::search::MovieSearch;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = MovieSearch::new("die hard".into());
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieSearch {
    /// Text query to search.
    pub query: String,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
    /// Whether to include adult (pornography) content in the results.
    pub include_adult: bool,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    pub region: Option<String>,
    pub year: Option<u16>,
    pub primary_release_year: Option<u16>,
}

impl MovieSearch {
    pub fn new(query: String) -> Self {
        Self {
            query,
            language: None,
            page: None,
            include_adult: false,
            region: None,
            year: None,
            primary_release_year: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }

    pub fn with_page(mut self, value: Option<u32>) -> Self {
        self.page = value;
        self
    }

    pub fn with_include_adult(mut self, value: bool) -> Self {
        self.include_adult = value;
        self
    }

    pub fn with_region(mut self, value: Option<String>) -> Self {
        self.region = value;
        self
    }

    pub fn with_year(mut self, value: Option<u16>) -> Self {
        self.year = value;
        self
    }

    pub fn with_primary_release_year(mut self, value: Option<u16>) -> Self {
        self.primary_release_year = value;
        self
    }
}

impl crate::prelude::Command for MovieSearch {
    type Output = crate::common::PaginatedResult<super::MovieShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(PATH)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = vec![("query", Cow::Borrowed(self.query.as_str()))];

        if let Some(language) = self.language.as_ref() {
            res.push(("language", Cow::Borrowed(language.as_str())));
        }
        if let Some(page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())));
        }
        if self.include_adult {
            res.push(("include_adult", Cow::Borrowed("true")));
        }
        if let Some(region) = self.region.as_ref() {
            res.push(("region", Cow::Borrowed(region.as_str())));
        }
        if let Some(year) = self.year {
            res.push(("year", Cow::Owned(year.to_string())));
        }
        if let Some(primary_release_year) = self.primary_release_year {
            res.push((
                "primary_release_year",
                Cow::Owned(primary_release_year.to_string()),
            ));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::MovieSearch;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = MovieSearch::new("Whatever".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/search-movie.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
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
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = MovieSearch::new("Whatever".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
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

        let cmd = MovieSearch::new("Whatever".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }

    #[tokio::test]
    async fn validation_error() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = MovieSearch::new("".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "".into()),
            ]))
            .with_status(422)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/validation-error.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let validation_err = err.as_validation_error().unwrap();
        assert_eq!(validation_err.errors.len(), 1);
    }

    // #[tokio::test]
    // async fn premature_end_of_line() {
    // let mut server = mockito::Server::new_async().await;
    // let client = Client::<ReqwestExecutor>::builder().with_api_key("secret".into()).with_base_url(server.url()).build().unwrap();

    //     let client = Client::<ReqwestExecutor>::new("secret".into()).with_base_url(mockito::server_url());
    //     let cmd = MovieSearch::new("game of thrones".into());

    //     let _m = mock("GET", super::PATH)
    //         .match_query(Matcher::AllOf(vec![
    //             Matcher::UrlEncoded("api_key".into(), "secret".into()),
    //             Matcher::UrlEncoded("query".into(), "game of thrones".into()),
    //         ]))
    //         .with_status(200)
    //         .with_header("content-type", "application/json;charset=utf-8")
    //         .with_body(include_str!("../../assets/search-tv-decoding-error.json"))
    //         .create_async().await;
    //     let result = cmd.execute(&client).await.unwrap();
    //     assert_eq!(result.page, 1);
    // }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieSearch;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;

    #[tokio::test]
    async fn search_rrrrrrr() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let cmd = MovieSearch::new("Rrrrrrr".into());

        let result = cmd.execute(&client).await.unwrap();
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
        let client = Client::<ReqwestExecutor>::new(secret);
        let cmd = MovieSearch::new("simpsons".into());

        let _result = cmd.execute(&client).await.unwrap();
    }
}
