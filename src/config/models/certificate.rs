#[derive(Clone, Debug, Deserialize)]
pub struct CertificateConfiguration {
    #[serde(rename = "caminho")]
    pub path: String,
    #[serde(rename = "senha")]
    pub password: String,
}
