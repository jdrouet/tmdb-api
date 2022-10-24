#[cfg(feature = "commands")]
pub mod list;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
