pub mod details;
pub mod genre;
pub mod search;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TVShowShort {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub origin_country: Vec<String>,
    pub overview: String,
    #[serde(with = "crate::util::date")]
    pub first_air_date: chrono::NaiveDate,
    pub genre_ids: Vec<u64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
}
