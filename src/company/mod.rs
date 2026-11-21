pub mod alternative_names;

pub mod details;

pub mod images;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct CompanyShort {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub origin_country: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Company {
    #[serde(flatten)]
    pub inner: CompanyShort,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub description: Option<String>,
    pub headquarters: String,
    pub homepage: String,
    pub parent_company: Option<CompanyShort>,
}
