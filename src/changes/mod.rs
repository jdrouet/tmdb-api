#[cfg(feature = "commands")]
pub mod list;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Change {
    pub id: Option<u64>,
    pub adult: Option<bool>,
}
