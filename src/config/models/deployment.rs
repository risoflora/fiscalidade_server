use crate::{
    errors::Errors,
    hashes::{sha256, sha256sum},
};

use super::{
    certificate::CertificateConfiguration, company::CompanyConfiguration,
    service::ServiceConfiguration,
};

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentConfiguration {
    #[serde(rename = "empresa")]
    pub company: CompanyConfiguration,
    #[serde(rename = "certificado")]
    pub certificate: CertificateConfiguration,
    #[serde(rename = "servico")]
    pub service: ServiceConfiguration,
}

impl DeploymentConfiguration {
    #[inline]
    pub fn token(&self) -> crate::Result<String> {
        let value = format!(
            "{document}{certificate}{password}{doc_model}{state}",
            document = self.company.document,
            certificate = sha256sum(&self.certificate.path).map_err(|error| {
                Errors::CannotLoadCertificate {
                    path: self.certificate.path.clone(),
                    error,
                }
            })?,
            password = self.certificate.password,
            doc_model = self.service.doc_model,
            state = self.service.state
        );
        let token = sha256(&value);
        Ok(token)
    }
}
