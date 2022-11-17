#[cfg(feature = "commands")]
pub mod alternative_titles;
#[cfg(feature = "commands")]
pub mod changes;
#[cfg(feature = "commands")]
pub mod credits;
#[cfg(feature = "commands")]
pub mod details;
#[cfg(feature = "commands")]
pub mod external_ids;
#[cfg(feature = "commands")]
pub mod images;
#[cfg(feature = "commands")]
pub mod keywords;
#[cfg(feature = "commands")]
pub mod latest;
#[cfg(feature = "commands")]
pub mod lists;
#[cfg(feature = "commands")]
pub mod now_playing;
#[cfg(feature = "commands")]
pub mod popular;
#[cfg(feature = "commands")]
pub mod recommendations;
#[cfg(feature = "commands")]
pub mod release_dates;
#[cfg(feature = "commands")]
pub mod reviews;
#[cfg(feature = "commands")]
pub mod search;
#[cfg(feature = "commands")]
pub mod similar;
#[cfg(feature = "commands")]
pub mod translations;
#[cfg(feature = "commands")]
pub mod videos;
#[cfg(feature = "commands")]
pub mod watch_providers;

use crate::common::country::Country;
use crate::common::language::Language;
use crate::common::status::Status;
use crate::company::CompanyShort;
use crate::genre::Genre;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieShort {
    #[serde(flatten)]
    pub inner: MovieBase,
    pub genre_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    #[serde(flatten)]
    pub inner: MovieBase,
    pub budget: u64,
    pub genres: Vec<Genre>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
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

#[cfg(test)]
mod tests {

    #[test]
    fn serialize() {
        let origin = include_str!("../../assets/movie-details-success.json");
        let movie: super::Movie = serde_json::from_str(origin).unwrap();
        let serial = serde_json::to_string_pretty(&movie).unwrap();
        println!("serial: {}", serial);
        let expected: super::Movie = serde_json::from_str(&serial).unwrap();
        assert_eq!(movie, expected);
    }
}
