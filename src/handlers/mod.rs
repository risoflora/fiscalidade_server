use crate::{json::Json, version::Version};

pub mod dfe;
pub mod payload;

pub async fn version() -> Json<Version> {
    Json(Version::default())
}
