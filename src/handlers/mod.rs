use crate::{json::Json, version::Version};

pub mod dfe;

pub async fn version() -> Json<Version> {
    Json(Version::default())
}
