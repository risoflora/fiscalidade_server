#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfiguration {
    #[serde(rename = "tipo")]
    pub doc_model: String,
    #[serde(rename = "estado")]
    pub state: String,
    #[serde(rename = "ambiente")]
    pub environment: String,
}
