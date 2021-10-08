#[derive(Clone, Debug, Deserialize)]
pub struct CompanyConfiguration {
    #[serde(rename = "documento")]
    pub document: String,
}
