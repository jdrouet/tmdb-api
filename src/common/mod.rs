pub mod country;
pub mod genre;
pub mod language;
pub mod status;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}
