pub mod country;
pub mod credits;
pub mod image;
pub mod keyword;
pub mod language;
pub mod release_date;
pub mod status;
pub mod video;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityResults<V> {
    pub id: u64,
    pub results: V,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Results<V> {
    pub results: V,
}
