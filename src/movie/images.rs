use crate::common::image::Image;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Get the images that belong to a movie.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::images::MovieImages;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieImages::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieImages {
    /// ID of the movie
    pub movie_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl MovieImages {
    pub fn new(movie_id: u64) -> Self {
        Self {
            movie_id,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieImagesResult {
    pub id: u64,
    pub backdrops: Vec<Image>,
    pub posters: Vec<Image>,
}

impl crate::prelude::Command for MovieImages {
    type Output = MovieImagesResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/images", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(ref language) = self.language {
            vec![("language", Cow::Borrowed(language))]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MovieImages;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieImages::new(550);

        let _m = mock("GET", "/movie/550/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-images.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieImages::new(42);

        let _m = mock("GET", "/movie/42/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieImages::new(42);

        let _m = mock("GET", "/movie/42/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieImages;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = MovieImages::new(550);

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }
}
