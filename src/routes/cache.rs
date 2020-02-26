use rocket::get;
use rocket_contrib::json::JsonValue;

use super::auth::AuthAdmin;
use super::ApiError;
use crate::db::{self, Conn};

#[get("/cache")]
pub fn list(conn: Conn, _auth: AuthAdmin) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::cache::list(&conn)?))
}
