use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};

use crate::db::{self, taxpayer};
use crate::models::taxpayer::QueryableTaxpayer;

pub struct Auth {
    pub id: i64,
    pub token: String,
}

impl Auth {
    pub fn enter<'a, 'r>(
        request: &'a Request<'r>,
    ) -> request::Outcome<(db::Conn, QueryableTaxpayer), ()> {
        let tokens: Vec<_> = request.headers().get("x-auth-token").collect();
        if tokens.len() != 1 {
            return Outcome::Failure((Status::Forbidden, ()));
        }
        let token = tokens[0];
        let conn: db::Conn = request.guard().unwrap();
        let taxpayer = match taxpayer::by_token(&conn, &token) {
            Ok(taxpayer) => taxpayer,
            Err(_) => return Outcome::Failure((Status::Forbidden, ())),
        };
        if !taxpayer.active {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        rocket::Outcome::Success((conn, taxpayer))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let (_, taxpayer) = Auth::enter(request)?;
        Outcome::Success(Self {
            id: taxpayer.id,
            token: taxpayer.token,
        })
    }
}

pub struct AuthAdmin {
    pub id: i64,
    pub token: String,
    pub manager: bool,
}

impl AuthAdmin {
    pub fn enter<'a, 'r>(
        request: &'a Request<'r>,
    ) -> request::Outcome<(db::Conn, QueryableTaxpayer, bool), ()> {
        let (conn, taxpayer) = Auth::enter(request)?;
        let is_manager = taxpayer.manager;
        if !is_manager {
            return Outcome::Failure((Status::Forbidden, ()));
        }
        rocket::Outcome::Success((conn, taxpayer, is_manager))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthAdmin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let (_, taxpayer, is_manager) = AuthAdmin::enter(request)?;
        Outcome::Success(Self {
            id: taxpayer.id,
            token: taxpayer.token,
            manager: is_manager,
        })
    }
}
