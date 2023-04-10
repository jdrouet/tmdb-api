#[cfg(feature = "commands")]
pub mod details;
#[cfg(feature = "commands")]
pub mod search;
#[cfg(feature = "commands")]
pub mod similar;

pub mod episode;
pub mod season;

use crate::common::country::Country;
use crate::common::language::Language;
use crate::company::CompanyShort;
use crate::genre::Genre;
use crate::people::PersonShort;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TVShowBase {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub origin_country: Vec<String>,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(with = "crate::util::empty_date")]
    pub first_air_date: Option<chrono::NaiveDate>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    #[serde(default)]
    pub adult: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TVShowShort {
    #[serde(flatten)]
    pub inner: TVShowBase,
    pub genre_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeShort {
    #[serde(with = "crate::util::date")]
    pub air_date: chrono::NaiveDate,
    pub episode_number: u64,
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub overview: Option<String>,
    pub production_code: String,
    pub season_number: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    #[serde(flatten)]
    pub inner: EpisodeShort,
    //
    pub crew: Vec<PersonShort>,
    pub guest_stars: Vec<PersonShort>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SeasonBase {
    #[serde(with = "crate::util::optional_date")]
    pub air_date: Option<chrono::NaiveDate>,
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SeasonShort {
    #[serde(flatten)]
    pub inner: SeasonBase,
    //
    pub episode_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Season {
    pub _id: String,
    #[serde(flatten)]
    pub inner: SeasonBase,
    pub episodes: Vec<Episode>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
    #[serde(with = "crate::util::optional_date")]
    pub last_air_date: Option<chrono::NaiveDate>,
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
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub tagline: Option<String>,
    #[serde(rename = "type")]
    pub ttype: String,
}
