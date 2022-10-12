use crate::common::company::Company;
use crate::common::country::Country;
use crate::common::genre::Genre;
use crate::common::language::Language;
use crate::common::status::Status;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub budget: u64,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub id: u64,
    pub imdb_id: Option<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<Company>,
    pub production_countries: Vec<Country>,
    #[serde(with = "crate::util::date")]
    pub release_date: chrono::NaiveDate,
    pub revenue: u64,
    pub runtime: Option<u64>,
    pub spoken_languages: Vec<Language>,
    pub status: Status,
    pub tagline: Option<String>,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u64,
}

/// Command to search for movies
#[derive(Clone, Debug, Default)]
pub struct MovieDetails {
    /// ID of the movie.
    pub movie_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl MovieDetails {
    pub fn new(movie_id: u64) -> Self {
        Self {
            movie_id,
            language: None,
        }
    }
}

impl crate::prelude::Command for MovieDetails {
    type Output = Item;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(language) = self.language.as_ref() {
            vec![("language", Cow::Borrowed(language.as_str()))]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MovieDetails;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/movie/550")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-details-success.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = MovieDetails::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/movie/550")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieDetails::new(550).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/movie/550")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieDetails::new(550).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieDetails;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieDetails::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }
}
