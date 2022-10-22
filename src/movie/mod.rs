pub mod details;
pub mod search;
pub mod similar;

use crate::common::country::Country;
use crate::common::language::Language;
use crate::common::status::Status;
use crate::company::CompanyShort;
use crate::genre::Genre;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieBase {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: String,
    #[serde(default, with = "crate::util::optional_date")]
    pub release_date: Option<chrono::NaiveDate>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    pub video: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieShort {
    #[serde(flatten)]
    pub inner: MovieBase,
    pub genre_ids: Vec<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Movie {
    #[serde(flatten)]
    pub inner: MovieBase,
    pub budget: u64,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub imdb_id: Option<String>,
    pub production_companies: Vec<CompanyShort>,
    pub production_countries: Vec<Country>,
    pub revenue: u64,
    pub runtime: Option<u64>,
    pub spoken_languages: Vec<Language>,
    pub status: Status,
    pub tagline: Option<String>,
}
