use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};

use super::auth::Auth;
use crate::db::{service, taxpayer_service};

pub struct ServiceAuth {
    pub id: i64,
    pub service: String,
    pub certificate: Vec<u8>,
    pub password: String,
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for ServiceAuth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let (conn, taxpayer) = Auth::enter(request)?;
        let path = &request.uri().path()[(mount_path!().len())..];
        let paths: Vec<_> = (&path[1..]).split("/").collect();
        if paths.len() < 1 {
            return Outcome::Failure((Status::Forbidden, ()));
        }
        let service = match service::by_slug(&conn, paths[0]) {
            Ok(service) => service,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };
        if !service.active {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let taxpayer_and_service =
            match taxpayer_service::by_taxpayer_and_service(&conn, taxpayer.id, service.id) {
                Ok(taxpayer_and_service) => taxpayer_and_service,
                Err(_) => return Outcome::Failure((Status::Forbidden, ())),
            };
        if !taxpayer_and_service.allowed_at.is_some() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let certificate = match base64::decode(&taxpayer.certificate) {
            Ok(certificate) => certificate,
            Err(_) => return Outcome::Failure((Status::FailedDependency, ())),
        };
        Outcome::Success(Self {
            id: taxpayer.id,
            service: service.slug,
            certificate: certificate,
            password: taxpayer.certificate_password,
            token: taxpayer.token,
        })
    }
}
