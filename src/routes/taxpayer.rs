use rocket_contrib::json::{Json, JsonValue};

use super::{auth::AuthAdmin, ApiError};
use crate::db::{self, Conn};
use crate::models::taxpayer::{InsertableTaxpayer, UpdatableTaxpayer};

#[post("/taxpayers/manager")]
pub fn create_manager(conn: Conn) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer::create_manager(&conn)?))
}

#[post("/taxpayers", data = "<taxpayer>")]
pub fn create(conn: Conn, mut taxpayer: Json<InsertableTaxpayer>) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer::create(&conn, &mut taxpayer)?))
}

#[put("/taxpayers/<id>", data = "<taxpayer>")]
pub fn update(
    conn: Conn,
    _auth: AuthAdmin,
    id: i64,
    taxpayer: Json<UpdatableTaxpayer>,
) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer::update(&conn, id, &taxpayer)?))
}

#[delete("/taxpayers/<id>")]
pub fn delete(conn: Conn, _auth: AuthAdmin, id: i64) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer::delete(&conn, id)?))
}

#[get("/taxpayers")]
pub fn list(conn: Conn, _auth: AuthAdmin) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer::list(&conn)?))
}
