#[derive(Clone, Debug, serde::Deserialize)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub origin_country: Option<String>,
}
