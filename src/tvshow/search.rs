use std::borrow::Cow;

const PATH: &str = "/search/tv";

/// Command to search for TV shows.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::search::TVShowSearch;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = TVShowSearch::new("simpsons".into());
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowSearch {
    /// Text query to search.
    pub query: String,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
    /// Whether to include adult (pornography) content in the results.
    pub include_adult: bool,
    pub first_air_date_year: Option<u16>,
}

impl TVShowSearch {
    pub fn new(query: String) -> Self {
        Self {
            query,
            language: None,
            page: None,
            include_adult: false,
            first_air_date_year: None,
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

    pub fn with_first_air_date_year(mut self, value: Option<u16>) -> Self {
        self.first_air_date_year = value;
        self
    }
}

impl crate::prelude::Command for TVShowSearch {
    type Output = crate::common::PaginatedResult<super::TVShowShort>;

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
        if let Some(first_air_date_year) = self.first_air_date_year {
            res.push((
                "first_air_date_year",
                Cow::Owned(first_air_date_year.to_string()),
            ));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowSearch;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = TVShowSearch::new("Whatever".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/search-tv.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
        assert!(result.total_pages > 0);
        assert!(result.total_results > 0);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "Game of Thrones");
    }

    /// Referring to issue https://github.com/jdrouet/tmdb-api/issues/25
    #[tokio::test]
    async fn fix_issue_25() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = TVShowSearch::new("rick and morty".into());

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "rick and morty".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/search-tv-rick-and-morty.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 2);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "Rick and Morty");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = TVShowSearch::new("Whatever".into());

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

        let cmd = TVShowSearch::new("Whatever".into());

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
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowSearch;

    #[tokio::test]
    async fn search_simpsons() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = TVShowSearch::new("simpsons".into());

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert!(result.results.len() > 1);
        assert!(result.total_pages > 0);
        assert!(result.total_results > 1);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "The Simpsons");
    }
}
