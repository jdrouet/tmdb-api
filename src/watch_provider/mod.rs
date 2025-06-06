pub mod list;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WatchProvider {
    pub provider_id: u64,
    pub provider_name: String,
    pub display_priority: u64,
    pub logo_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocatedWatchProvider {
    pub link: String,
    #[serde(default)]
    pub flatrate: Vec<WatchProvider>,
    #[serde(default)]
    pub rent: Vec<WatchProvider>,
    #[serde(default)]
    pub buy: Vec<WatchProvider>,
}
