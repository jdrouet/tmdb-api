use std::borrow::Cow;

const PATH: &str = "/search/tv";

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Params<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
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
    /// Search the first air date and all episode air dates. Valid values are: 1000..9999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    /// Search only the first air date. Valid values are: 1000..9999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_air_date_year: Option<u16>,
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

    pub fn set_first_air_date_year(&mut self, value: u16) {
        self.first_air_date_year = Some(value);
    }

    pub fn with_first_air_date_year(mut self, value: u16) -> Self {
        self.set_first_air_date_year(value);
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
    /// Command to search for tvshows
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.search_tvshows("simpsons", &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn search_tvshows<'a>(
        &self,
        query: impl Into<Cow<'a, str>>,
        params: &Params<'a>,
    ) -> crate::Result<crate::common::PaginatedResult<super::TVShowShort>> {
        self.execute(
            PATH,
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
    use crate::client::Client;
    use crate::client::reqwest::reqwest::Client as ReqwestClient;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

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
        let result = client
            .search_tvshows("Whatever", &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
        assert!(result.total_pages > 0);
        assert!(result.total_results > 0);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "Game of Thrones");
    }

    /// Refering to issue https://github.com/jdrouet/tmdb-api/issues/25
    #[tokio::test]
    async fn fix_issue_25() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

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
        let result = client
            .search_tvshows("rick and morty", &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 2);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "Rick and Morty");
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
        let err = client
            .search_tvshows("Whatever", &Default::default())
            .await
            .unwrap_err();
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
        let err = client
            .search_tvshows("Whatever", &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn search_simpsons() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let result = client
            .search_tvshows("simpsons", &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert!(result.results.len() > 1);
        assert!(result.total_pages > 0);
        assert!(result.total_results > 1);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "The Simpsons");
    }
}
