#[cfg(feature = "commands")]
pub mod list;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certification {
    pub certification: String,
    pub meaning: String,
    pub order: usize,
}
