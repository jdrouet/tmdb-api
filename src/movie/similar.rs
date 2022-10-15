use std::borrow::Cow;

/// Command to get similar movies to a movie
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::similar::GetSimilarMovies;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = GetSimilarMovies::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct GetSimilarMovies {
    /// ID of the movie
    pub movie_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
}

impl GetSimilarMovies {
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

impl crate::prelude::Command for GetSimilarMovies {
    type Output = crate::common::PaginatedResult<super::MovieShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/similar", self.movie_id))
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
    use super::GetSimilarMovies;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = GetSimilarMovies::new(42);

        let _m = mock("GET", "/movie/42/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-similar-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.total_pages, 9);
        assert_eq!(result.total_results, 168);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.title, "Darna: The Return");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = GetSimilarMovies::new(42);

        let _m = mock("GET", "/movie/42/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = GetSimilarMovies::new(42);

        let _m = mock("GET", "/movie/42/similar")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::GetSimilarMovies;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = GetSimilarMovies::new(106912);

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 20);
        assert_eq!(result.total_pages, 500);
        assert_eq!(result.total_results, 10000);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.id, 1278);
    }
}
