use axum::Json;
use fiscalidade::{Ambiente, Dfe, Modelo, Pkcs12Certificate, Uf, WebServices};

use crate::{
    config::models::deployment::DeploymentConfiguration, errors::Errors, response::Response,
    version::Version,
};

pub async fn version() -> Json<Version> {
    Json(Version::default())
}

pub async fn status_servico(deployment: DeploymentConfiguration) -> crate::Result<Json<Response>> {
    let webservices = WebServices::from_embedded()?;
    let pkcs12 = Pkcs12Certificate::from_file(
        &deployment.certificate.path,
        &deployment.certificate.password,
    )
    .await?;
    let document = deployment.company.document;
    let dfe = Dfe::new().set_webservices(webservices).set_pkcs12(pkcs12);
    let xml = dfe
        .status_servico(
            Modelo::from_str(&deployment.service.doc_model).ok_or(
                Errors::InvalidConfiguration {
                    document: document.clone(),
                    configuration: stringify!(deployment.service.doc_model).to_string(),
                },
            )?,
            Uf::from_str(&deployment.service.state).ok_or(Errors::InvalidConfiguration {
                document: document.clone(),
                configuration: stringify!(deployment.service.state).to_string(),
            })?,
            Ambiente::from_str(&deployment.service.environment).ok_or(
                Errors::InvalidConfiguration {
                    document: document.clone(),
                    configuration: stringify!(deployment.service.environment).to_string(),
                },
            )?,
        )
        .await?;
    Ok(Json(Response::from_xml(xml.to_string())))
}
