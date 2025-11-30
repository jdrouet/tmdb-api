pub mod search;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "media_type")]
pub enum MultiSearchResult {
    #[serde(rename = "movie")]
    Movie(crate::movie::MovieShort),

    #[serde(rename = "tv")]
    TVShow(crate::tvshow::TVShowShort),

    #[serde(rename = "person")]
    Person(crate::people::PersonShort),
}
