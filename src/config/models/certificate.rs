#[derive(Clone, Debug, Deserialize)]
pub struct CertificateConfiguration {
    pub path: String,
    pub password: String,
}
