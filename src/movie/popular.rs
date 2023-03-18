use crate::common::PaginatedResult;
use std::borrow::Cow;

/// Get a list of the current popular movies on TMDB. This list updates daily.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::popular::MoviePopular;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let result = MoviePopular::default().execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MoviePopular {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Specify which page to query.
    pub page: Option<u32>,
    /// Specify a ISO 3166-1 code to filter release dates. Must be uppercase.
    pub region: Option<String>,
}

impl MoviePopular {
    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }

    pub fn with_page(mut self, value: Option<u32>) -> Self {
        self.page = value;
        self
    }

    pub fn with_region(mut self, value: Option<String>) -> Self {
        self.region = value;
        self
    }
}

impl crate::prelude::Command for MoviePopular {
    type Output = PaginatedResult<super::MovieShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed("/movie/popular")
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::new();
        if let Some(ref language) = self.language {
            res.push(("language", Cow::Borrowed(language.as_str())))
        }
        if let Some(ref page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())))
        }
        if let Some(ref region) = self.region {
            res.push(("region", Cow::Borrowed(region.as_str())))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::MoviePopular;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-popular.json"))
            .create_async()
            .await;

        let result = MoviePopular::default().execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = MoviePopular::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = MoviePopular::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MoviePopular;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let _result = MoviePopular::default().execute(&client).await.unwrap();
    }
}
