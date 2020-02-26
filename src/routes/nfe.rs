use anyhow::anyhow;
use fiscalidade::{Ambiente, Dfe, DfeError, Documento, Pkcs12Certificate, Tipo, Uf, Xml};
use rocket::State;
use rocket_contrib::json::JsonValue;

use super::service_auth::ServiceAuth;
use super::ApiError;
use crate::db::{cache, Conn};
use crate::AppData;

fn dfe_request(
    auth: ServiceAuth,
    data: State<'_, AppData>,
    uf: &str,
    ambiente: &str,
) -> Result<(Uf, Ambiente, Dfe), ApiError> {
    let uf = match Uf::from_str(&uf) {
        Some(uf) => uf,
        None => return Err(anyhow!("UF inválida").into()),
    };
    let ambiente = match Ambiente::from_str(&ambiente) {
        Some(ambiente) => ambiente,
        None => return Err(anyhow!("Ambiente inválido").into()),
    };
    let service = match Tipo::from_str(&auth.service) {
        Some(service) => service,
        None => return Err(anyhow!("Serviço inválido").into()),
    };
    let pkcs12 = Pkcs12Certificate::from_bytes(&auth.certificate, &auth.password)?;
    Ok((
        uf,
        ambiente,
        Dfe::new(service)
            .set_webservices(data.webservices.clone())
            .set_pkcs12(pkcs12),
    ))
}

pub fn cache_xml<F>(conn: &Conn, key: &str, get_value: F) -> Result<Xml, ApiError>
where
    F: FnOnce() -> Result<Xml, DfeError>,
{
    let key = key.to_lowercase();
    let xml = match cache::lookup_value(conn, &key)? {
        Some(xml) => xml,
        None => {
            let xml = get_value()?;
            cache::set_value(conn, &key, &xml.0)?
        }
    };
    Ok(Xml(xml))
}

pub type ServiceResult = Result<JsonValue, ApiError>;

#[get("/status-servico/<uf>/<ambiente>")]
pub fn status_servico(
    auth: ServiceAuth,
    data: State<'_, AppData>,
    uf: String,
    ambiente: String,
) -> ServiceResult {
    let (uf, ambiente, dfe) = dfe_request(auth, data, &uf, &ambiente)?;
    Ok(json_ok!(dfe.status_servico(uf, ambiente)?.to_string()))
}

#[get("/consultar-cadastro/<uf>/<ambiente>/<tipo_documento>/<documento>")]
pub fn consultar_cadastro(
    conn: Conn,
    auth: ServiceAuth,
    data: State<'_, AppData>,
    uf: String,
    ambiente: String,
    tipo_documento: String,
    documento: String,
) -> ServiceResult {
    let documento = match tipo_documento.to_lowercase().as_str() {
        "ie" => Documento::from_ie(&documento),
        "cpf" => Documento::from_cpf(&documento),
        "cnpj" => Documento::from_cnpj(&documento),
        _ => return Err(anyhow!("Tipo inválido de documento").into()),
    };
    let (uf, ambiente, dfe) = dfe_request(auth, data, &uf, &ambiente)?;
    let key = format!(
        "{}_{}_consulta_cadastro_{}_{}",
        uf, ambiente, tipo_documento, documento
    );
    let xml = cache_xml(&conn, &key, || {
        dfe.consultar_cadastro(uf, ambiente, documento)
    })?;
    Ok(json_ok!(xml.to_string()))
}

#[get("/consultar-xml/<uf>/<ambiente>/<chave>")]
pub fn consultar_xml(
    conn: Conn,
    auth: ServiceAuth,
    data: State<'_, AppData>,
    uf: String,
    ambiente: String,
    chave: String,
) -> ServiceResult {
    let (uf, ambiente, dfe) = dfe_request(auth, data, &uf, &ambiente)?;
    let key = format!("{}_{}_consulta_xml_{}", uf, ambiente, chave);
    let xml = cache_xml(&conn, &key, || dfe.consultar_xml(uf, ambiente, &chave))?;
    Ok(json_ok!(xml.to_string()))
}
