use std::borrow::Cow;

pub mod country;
pub mod credits;
pub mod image;
pub mod keyword;
pub mod language;
pub mod release_date;
pub mod status;
pub mod video;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityResults<V> {
    pub id: u64,
    pub results: V,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Results<V> {
    pub results: V,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct LanguageParams<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Cow<'a, str>>,
}

impl<'a> LanguageParams<'a> {
    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}
