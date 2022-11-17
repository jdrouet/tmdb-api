use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub site: String,
    pub key: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub size: u64,
    pub iso_639_1: String,
    pub iso_3166_1: String,
}
