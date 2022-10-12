#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
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
