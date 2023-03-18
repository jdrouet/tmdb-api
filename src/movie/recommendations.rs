use crate::common::PaginatedResult;
use std::borrow::Cow;

/// Get a list of recommended movies for a movie.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::recommendations::MovieRecommendations;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieRecommendations::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieRecommendations {
    /// ID of the movie.
    pub movie_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Specify which page to query.
    pub page: Option<u32>,
}

impl MovieRecommendations {
    pub fn new(movie_id: u64) -> Self {
        Self {
            movie_id,
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

impl crate::prelude::Command for MovieRecommendations {
    type Output = PaginatedResult<super::MovieShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/recommendations", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::with_capacity(2);
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
    use super::MovieRecommendations;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/550/recommendations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-recommendations.json"))
            .create_async()
            .await;

        let result = MovieRecommendations::new(550)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/550/recommendations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = MovieRecommendations::new(550)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::new("secret".into()).with_base_url(server.url());

        let _m = server
            .mock("GET", "/movie/550/recommendations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = MovieRecommendations::new(550)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieRecommendations;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieRecommendations::new(550)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }
}
