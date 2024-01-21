use serde::{Deserialize, Serialize};

#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
