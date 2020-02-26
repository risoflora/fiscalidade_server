use std::string::FromUtf8Error;

use fiscalidade::{DfeError, Pkcs12CertificateError};
use rocket::{
    get,
    http::Status,
    request::Request,
    response::{self, status, Responder},
};
use rocket_contrib::json::JsonValue;
use thiserror::Error;

pub mod auth;
pub mod cache;
pub mod nfe;
pub mod service_auth;
pub mod services;
pub mod taxpayer;
pub mod taxpayer_service;

use crate::db;
use crate::utils::{Info, Version};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Db(#[from] db::Error),
    #[error(transparent)]
    Pkcs12(#[from] Pkcs12CertificateError),
    #[error(transparent)]
    Dfe(#[from] DfeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request<'_>) -> response::Result<'r> {
        status::Custom(Status::UnprocessableEntity, json_error!(self.to_string())).respond_to(req)
    }
}

#[get("/version")]
pub fn version() -> JsonValue {
    json_ok!(Version::new())
}

#[get("/info")]
pub fn info() -> JsonValue {
    json_ok!(Info::new())
}
