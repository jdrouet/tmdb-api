use crate::common::keyword::Keyword;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Get the keywords that have been added to a movie.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::keywords::MovieKeywords;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieKeywords::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieKeywords {
    /// ID of the movie
    pub movie_id: u64,
}

impl MovieKeywords {
    pub fn new(movie_id: u64) -> Self {
        Self { movie_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieKeywordsResult {
    pub id: u64,
    pub keywords: Vec<Keyword>,
}

impl crate::prelude::Command for MovieKeywords {
    type Output = MovieKeywordsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/keywords", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MovieKeywords;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let cmd = MovieKeywords::new(550);

        let _m = server
            .mock("GET", "/movie/550/keywords")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-keywords.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let cmd = MovieKeywords::new(42);

        let _m = server
            .mock("GET", "/movie/42/keywords")
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

        let cmd = MovieKeywords::new(42);

        let _m = server
            .mock("GET", "/movie/42/keywords")
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
    use super::MovieKeywords;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = MovieKeywords::new(550);

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }
}
