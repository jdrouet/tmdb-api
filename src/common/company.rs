#[derive(Clone, Debug, serde::Deserialize)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    pub origin_country: String,
}
