#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: u64,
    pub iso_639_1: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
    pub width: u64,
}
