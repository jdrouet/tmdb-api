#[cfg(feature = "commands")]
pub mod alternative_names;
#[cfg(feature = "commands")]
pub mod details;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyShort {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub origin_country: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Company {
    #[serde(flatten)]
    pub inner: CompanyShort,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub description: Option<String>,
    pub headquarters: String,
    pub homepage: String,
    pub parent_company: Option<CompanyShort>,
}
