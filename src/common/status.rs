#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub enum Status {
    Rumored,
    Planned,
    #[serde(rename = "In Production")]
    InProduction,
    #[serde(rename = "Post Production")]
    PostProduction,
    Released,
    Canceled,
}
