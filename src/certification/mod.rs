#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certification {
    pub certification: String,
    pub meaning: String,
    pub order: usize,
}
