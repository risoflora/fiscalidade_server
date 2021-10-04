use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    extract::rejection::ExtensionRejection,
    http::{Response as AxumResponse, StatusCode},
    response::IntoResponse,
    Json,
};
use fiscalidade::{DfeError, Pkcs12CertificateError, WebServicesError};
use serde_json::json;
use thiserror::Error;

#[derive(Clone, Debug, Serialize)]
pub struct Response {
    pub xml: String,
}

#[derive(Debug, Error)]
pub enum ResponseError {
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
    #[error("Invalid configuration for document {document} {configuration}")]
    InvalidConfiguration {
        document: String,
        configuration: String,
    },
    #[error("Deployment not found for token {0}")]
    DeploymentNotFound(String),
}

pub type ResponseResult<T> = Result<T, ResponseError>;

impl Response {
    pub fn from_xml(xml: String) -> Self {
        Self { xml }
    }
}

impl IntoResponse for ResponseError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> AxumResponse<Self::Body> {
        let status = match self {
            ResponseError::MissingToken | ResponseError::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
