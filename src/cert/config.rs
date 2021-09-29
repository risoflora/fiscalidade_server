use super::Certificate;

#[derive(Clone, Debug, Deserialize)]
pub struct CertificateConfiguration {
    pub document: String,
    #[serde(flatten)]
    pub certificate: Certificate,
}
