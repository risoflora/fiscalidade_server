use crate::hashes::sha2;

use super::{
    certificate::CertificateConfiguration, company::CompanyConfiguration,
    service::ServiceConfiguration,
};

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentConfiguration {
    pub company: CompanyConfiguration,
    pub certificate: CertificateConfiguration,
    pub service: ServiceConfiguration,
}

impl DeploymentConfiguration {
    #[inline]
    pub fn token(&self) -> String {
        let value = format!("{}{}", self.company.document, self.certificate.password);
        sha2(&value)
    }
}
