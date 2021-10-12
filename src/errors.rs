use std::io;

use axum::extract::rejection::ExtensionRejection;
use fiscalidade::{DfeError, Pkcs12CertificateError, WebServicesError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    AxumExtensionRejection(#[from] ExtensionRejection),
    #[error(transparent)]
    FiscalidadeWebServicesError(#[from] WebServicesError),
    #[error(transparent)]
    FiscalidadePkcs12CertificateError(#[from] Pkcs12CertificateError),
    #[error(transparent)]
    FiscalidadeDfeError(#[from] DfeError),
    #[error("Cabeçalhos extraídos por outro extrator")]
    HeadersTakenByAnotherExtractor,
    #[error("Faltando token para autenticação")]
    MissingAuthToken,
    #[error("Token inválido para autenticação")]
    InvalidAuthToken,
    #[error("Não foi possível carregar certificado '{path}': {error}")]
    CannotLoadCertificate { path: String, error: io::Error },
    #[error("Configuração inválida para documento {document}: {configuration}")]
    InvalidConfiguration {
        document: String,
        configuration: String,
    },
    #[error("Implantação não encontrada para token {0}")]
    DeploymentNotFound(String),
    #[error("Faltando campo em payload: {0}")]
    MissingPayloadField(String),
}
