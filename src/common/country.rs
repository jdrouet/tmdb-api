#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Country {
    pub iso_3166_1: String,
    pub name: String,
}
