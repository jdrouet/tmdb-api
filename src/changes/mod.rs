pub mod list;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Change {
    pub id: Option<u64>,
    pub adult: Option<bool>,
}
