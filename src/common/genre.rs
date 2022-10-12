#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct GenreResult {
    pub genres: Vec<Genre>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
