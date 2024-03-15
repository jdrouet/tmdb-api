#[derive(Debug, Deserialize, Serialize)]
pub struct TVShowExternalIdsResult {
    pub id: u64,
    pub imdb_id: Option<String>,
    pub freebase_mid: Option<String>,
    pub freebase_id: Option<String>,
    pub tvdb_id: Option<u64>,
    pub tvrage_id: Option<u64>,
    pub wikidata_id: Option<String>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub twitter_id: Option<String>,
}
