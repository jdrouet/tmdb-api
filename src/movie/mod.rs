pub mod alternative_titles;

pub mod changes;

pub mod credits;

pub mod details;

pub mod external_ids;

pub mod images;

pub mod keywords;

pub mod latest;

pub mod lists;

pub mod now_playing;

pub mod popular;

pub mod recommendations;

pub mod release_dates;

pub mod reviews;

pub mod search;

pub mod similar;

pub mod top_rated;

pub mod translations;

pub mod upcoming;

pub mod videos;

pub mod watch_providers;

use crate::collection::CollectionBase;
use crate::common::country::Country;
use crate::common::language::Language;
use crate::common::status::Status;
use crate::company::CompanyShort;
use crate::genre::Genre;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieBase {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: String,
    #[serde(default, deserialize_with = "crate::util::empty_string::deserialize")]
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
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub imdb_id: Option<String>,
    pub belongs_to_collection: Option<CollectionBase>,
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
        let origin = include_str!("../../assets/movie-details.json");
        let movie: super::Movie = serde_json::from_str(origin).unwrap();
        let serial = serde_json::to_string_pretty(&movie).unwrap();
        println!("serial: {}", serial);
        let expected: super::Movie = serde_json::from_str(&serial).unwrap();
        assert_eq!(movie, expected);
    }
}
