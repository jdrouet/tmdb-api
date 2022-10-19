#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
