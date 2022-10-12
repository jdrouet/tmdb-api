use crate::common::genre::{Genre, GenreResult};
use std::borrow::Cow;

const PATH: &str = "/genre/movie/list";

/// Command to search for movie genres
#[derive(Clone, Debug, Default)]
pub struct MovieGenreList {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

#[async_trait::async_trait]
impl crate::prelude::Command for MovieGenreList {
    type Output = Vec<Genre>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(PATH)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(language) = self.language.as_ref() {
            vec![("language", Cow::Borrowed(language.as_str()))]
        } else {
            Vec::new()
        }
    }

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client
            .execute::<GenreResult>(self.path().as_ref(), self.params())
            .await
            .map(|res| res.genres)
    }
}

#[cfg(test)]
mod tests {
    use super::MovieGenreList;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieGenreList::default();

        let _m = mock("GET", super::PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/genre-movie-list-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieGenreList::default();

        let _m = mock("GET", super::PATH)
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
        let cmd = MovieGenreList::default();

        let _m = mock("GET", super::PATH)
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
    use super::MovieGenreList;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let mut cmd = MovieGenreList::default();
        cmd.language = Some("en-US".into());

        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
