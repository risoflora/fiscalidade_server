use rocket_contrib::json::JsonValue;

use super::ApiError;
use crate::db::{self, Conn};

#[get("/services")]
pub fn list(conn: Conn) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::service::list(&conn)?))
}
