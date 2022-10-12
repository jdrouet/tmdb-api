#[derive(Clone, Debug, serde::Deserialize)]
pub struct Country {
    pub iso_3166_1: String,
    pub name: String,
}
