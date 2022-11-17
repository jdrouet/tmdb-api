pub mod country;
pub mod credits;
pub mod image;
pub mod keyword;
pub mod language;
pub mod release_date;
pub mod status;
pub mod video;
pub mod watch_providers;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}
