#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}
