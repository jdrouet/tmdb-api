pub mod details;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct PersonShort {
    pub id: u64,
    pub credit_id: Option<String>,
    pub name: String,
    pub gender: Option<u64>,
    pub profile_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Person {
    #[serde(flatten)]
    pub inner: PersonShort,
    pub adult: bool,
    pub also_known_as: Vec<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub biography: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub birthday: Option<chrono::NaiveDate>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub deathday: Option<chrono::NaiveDate>,
    pub homepage: Option<String>,
    pub imdb_id: Option<String>,
    pub known_for_department: Option<String>,
    pub popularity: f64,
    pub place_of_birth: Option<String>,
    pub profile_path: Option<String>,
}
