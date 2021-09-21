use axum::Json;

use super::version::Version;

pub async fn version() -> Json<Version> {
    Json(Version::default())
}
