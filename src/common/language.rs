#[derive(Clone, Debug, serde::Deserialize)]
pub struct Language {
    pub iso_639_1: String,
    pub name: String,
}
