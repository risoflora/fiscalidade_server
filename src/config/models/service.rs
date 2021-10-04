#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfiguration {
    pub kind: String,
    pub state: String,
    pub environment: String,
}
