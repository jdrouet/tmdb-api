use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Get a list of translations that have been created for a movie.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::translations::MovieTranslations;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieTranslations::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieTranslations {
    /// ID of the movie.
    pub movie_id: u64,
}

impl MovieTranslations {
    pub fn new(movie_id: u64) -> Self {
        Self { movie_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranslationData {
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub title: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub overview: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub homepage: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub iso_3166_1: String,
    pub iso_639_1: String,
    pub name: String,
    pub english_name: String,
    pub data: TranslationData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieTranslationsResult {
    pub id: u64,
    pub translations: Vec<Translation>,
}

impl crate::prelude::Command for MovieTranslations {
    type Output = MovieTranslationsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/translations", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MovieTranslations;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/movie/550/translations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-translations-success.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = MovieTranslations::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
        assert_eq!(result.translations.len(), 41);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/movie/550/translations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieTranslations::new(550)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/movie/550/translations")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieTranslations::new(550)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieTranslations;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieTranslations::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }
}
