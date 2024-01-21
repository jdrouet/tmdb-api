#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub iso_639_1: String,
    pub name: String,
}
