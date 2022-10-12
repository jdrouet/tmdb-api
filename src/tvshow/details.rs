use crate::common::country::Country;
use crate::common::genre::Genre;
use crate::common::language::Language;
use crate::company::CompanyShort;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
pub struct EpisodeShort {
    #[serde(with = "crate::util::date")]
    pub air_date: chrono::NaiveDate,
    pub episode_number: u64,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: String,
    pub season_number: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SeasonShort {
    #[serde(with = "crate::util::date")]
    pub air_date: chrono::NaiveDate,
    pub episode_count: u64,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: String,
    pub season_number: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PersonShort {
    pub id: u64,
    pub credit_id: String,
    pub name: String,
    pub gender: u64,
    pub profile_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub backdrop_path: Option<String>,
    pub created_by: Vec<PersonShort>,
    pub episode_run_time: Vec<u64>,
    #[serde(with = "crate::util::date")]
    pub first_air_date: chrono::NaiveDate,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: u64,
    pub in_production: bool,
    pub languages: Vec<String>,
    #[serde(with = "crate::util::date")]
    pub last_air_date: chrono::NaiveDate,
    pub last_episode_to_air: Option<EpisodeShort>,
    pub name: String,
    pub next_episode_to_air: Option<EpisodeShort>,
    pub networks: Vec<CompanyShort>,
    pub number_of_episodes: u64,
    pub number_of_seasons: u64,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<CompanyShort>,
    pub production_countries: Vec<Country>,
    pub seasons: Vec<SeasonShort>,
    pub spoken_languages: Vec<Language>,
    pub status: String,
    pub tagline: String,
    #[serde(rename = "type")]
    pub ttype: String,
    pub vote_average: f64,
    pub vote_count: u64,
}

/// Command to search for tvshows
#[derive(Clone, Debug, Default)]
pub struct TVShowDetails {
    /// ID of the TV Show
    pub tv_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl TVShowDetails {
    pub fn new(tv_id: u64) -> Self {
        Self {
            tv_id,
            language: None,
        }
    }
}

impl crate::prelude::Command for TVShowDetails {
    type Output = Item;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}", self.tv_id))
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
    use super::TVShowDetails;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/tv/1399")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-details-success.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = TVShowDetails::new(1399).execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/tv/1399")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = TVShowDetails::new(1399).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/tv/1399")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = TVShowDetails::new(1399).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::TVShowDetails;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = TVShowDetails::new(1399).execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
    }
}
