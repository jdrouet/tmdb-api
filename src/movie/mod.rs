pub mod details;
pub mod genre;
pub mod search;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct MovieShort {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: String,
    #[serde(with = "crate::util::date")]
    pub release_date: chrono::NaiveDate,
    pub genre_ids: Vec<u64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    pub video: bool,
}
