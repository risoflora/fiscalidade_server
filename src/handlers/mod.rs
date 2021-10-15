use axum::{http::StatusCode, response::IntoResponse};
use serde_json::{json, Value};

use crate::json::Json;

pub mod dfe;

pub async fn version() -> Json<Value> {
    Json(json!({ "versao": env!("CARGO_PKG_VERSION") }))
}

pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "erro": "Serviço não encontrado" })),
    )
}
