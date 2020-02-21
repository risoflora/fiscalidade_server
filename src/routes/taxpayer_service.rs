use rocket_contrib::json::{Json, JsonValue};

use super::{
    auth::{Auth, AuthAdmin},
    ApiError,
};
use crate::db::{self, Conn};
use crate::models::taxpayer_service::InsertableTaxpayerService;

#[post("/taxpayers/services", data = "<taxpayer_service>")]
pub fn create(
    conn: Conn,
    _auth: Auth,
    taxpayer_service: Json<InsertableTaxpayerService>,
) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer_service::create(
        &conn,
        &taxpayer_service
    )?))
}

#[get("/taxpayers/services/unauthorized")]
pub fn unauthorized(conn: Conn, _auth: AuthAdmin) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer_service::unauthorized(&conn)?))
}

#[post("/taxpayers/services/authorize/<id>")]
pub fn authorize(conn: Conn, _auth: Auth, id: i64) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer_service::authorize(&conn, id)?))
}

#[put("/taxpayers/services/unauthorize/<id>")]
pub fn unauthorize(conn: Conn, _auth: Auth, id: i64) -> Result<JsonValue, ApiError> {
    Ok(json_ok!(db::taxpayer_service::unauthorize(&conn, id)?))
}
