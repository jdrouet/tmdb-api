pub mod details;
pub mod genre;
pub mod search;

use crate::common::country::Country;
use crate::common::genre::Genre;
use crate::common::language::Language;
use crate::company::CompanyShort;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TVShowBase {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub origin_country: Vec<String>,
    pub overview: String,
    #[serde(with = "crate::util::date")]
    pub first_air_date: chrono::NaiveDate,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TVShowShort {
    #[serde(flatten)]
    pub inner: TVShowBase,
    pub genre_ids: Vec<u64>,
}

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

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TVShow {
    #[serde(flatten)]
    pub inner: TVShowBase,
    //
    pub created_by: Vec<PersonShort>,
    pub episode_run_time: Vec<u64>,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub in_production: bool,
    pub languages: Vec<String>,
    #[serde(with = "crate::util::date")]
    pub last_air_date: chrono::NaiveDate,
    pub last_episode_to_air: Option<EpisodeShort>,
    pub next_episode_to_air: Option<EpisodeShort>,
    pub networks: Vec<CompanyShort>,
    pub number_of_episodes: u64,
    pub number_of_seasons: u64,
    pub production_companies: Vec<CompanyShort>,
    pub production_countries: Vec<Country>,
    pub seasons: Vec<SeasonShort>,
    pub spoken_languages: Vec<Language>,
    pub status: String,
    pub tagline: String,
    #[serde(rename = "type")]
    pub ttype: String,
}
