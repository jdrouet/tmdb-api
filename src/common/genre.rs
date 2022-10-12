#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct GenreResult<I> {
    pub genres: Vec<I>,
}
