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
    #[error("Headers has been taken by another extractor")]
    HeadersTakenByAnotherExtractor,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Cannot load certificate '{path}': {error}")]
    CannotLoadCertificate { path: String, error: io::Error },
    #[error("Invalid configuration for document {document}: {configuration}")]
    InvalidConfiguration {
        document: String,
        configuration: String,
    },
    #[error("Deployment not found for token {0}")]
    DeploymentNotFound(String),
}
