use axum::Json;

use crate::version::Version;

pub async fn version() -> Json<Version> {
    Json(Version::default())
}
