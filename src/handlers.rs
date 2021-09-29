use axum::{extract::Extension, Json};

use crate::{cert::store::CertificateStore, version::Version};

pub async fn version() -> Json<Version> {
    Json(Version::default())
}

pub async fn status_servico(Extension(_certs): Extension<CertificateStore>) -> Json<Version> {
    Json::default()
}
