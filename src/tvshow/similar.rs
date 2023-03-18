use std::borrow::Cow;

/// Command to get similar tvshows
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::similar::GetSimilarTVShows;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = GetSimilarTVShows::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct GetSimilarTVShows {
    /// ID of the tvshow
    pub tvshow_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
}

impl GetSimilarTVShows {
    pub fn new(tvshow_id: u64) -> Self {
        Self {
            tvshow_id,
            language: None,
            page: None,
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
}

impl crate::prelude::Command for GetSimilarTVShows {
    type Output = crate::common::PaginatedResult<super::TVShowShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/similar", self.tvshow_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = vec![];

        if let Some(language) = self.language.as_ref() {
            res.push(("language", Cow::Borrowed(language.as_str())));
        }
        if let Some(page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::GetSimilarTVShows;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let cmd = GetSimilarTVShows::new(42);

        let _m = server
            .mock("GET", "/tv/42/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-similar.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 20);
        assert_eq!(result.total_pages, 500);
        assert_eq!(result.total_results, 10000);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.name, "The Great Queen Seondeok");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let cmd = GetSimilarTVShows::new(42);

        let _m = server
            .mock("GET", "/tv/42/similar")
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
        let client = Client::new("secret".into()).with_base_url(server.url());

        let cmd = GetSimilarTVShows::new(42);

        let _m = server
            .mock("GET", "/tv/42/similar")
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
    use super::GetSimilarTVShows;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = GetSimilarTVShows::new(1399);

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 20);
        assert_eq!(result.total_pages, 500);
        assert_eq!(result.total_results, 10000);
    }
}
