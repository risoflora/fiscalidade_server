use axum::Json;
use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Tipo, Uf, WebServices};

use crate::{
    config::models::deployment::DeploymentConfiguration,
    response::{Response, ResponseError, ResponseResult},
    version::Version,
};

pub async fn version() -> Json<Version> {
    Json(Version::default())
}

pub async fn status_servico(deployment: DeploymentConfiguration) -> ResponseResult<Json<Response>> {
    let webservices = WebServices::from_embedded()?;
    let pkcs12 = Pkcs12Certificate::from_file(
        &deployment.certificate.path,
        &deployment.certificate.password,
    )
    .await?;
    let document = deployment.company.document;
    let dfe = Dfe::new(Tipo::from_str(&deployment.service.kind).ok_or(
        ResponseError::InvalidConfiguration {
            document: document.clone(),
            configuration: "deployment.service.kind".to_string(),
        },
    )?)
    .set_webservices(webservices)
    .set_pkcs12(pkcs12);
    let xml = dfe
        .status_servico(
            Uf::from_str(&deployment.service.state).ok_or(ResponseError::InvalidConfiguration {
                document: document.clone(),
                configuration: "deployment.service.state".to_string(),
            })?,
            Ambiente::from_str(&deployment.service.environment).ok_or(
                ResponseError::InvalidConfiguration {
                    document: document.clone(),
                    configuration: "deployment.service.environment".to_string(),
                },
            )?,
        )
        .await?;
    Ok(Json(Response::from_xml(xml.to_string())))
}
