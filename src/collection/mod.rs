use serde::{Deserialize, Serialize};

#[cfg(feature = "commands")]
pub mod details;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CollectionBase {
    pub id: u64,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}
