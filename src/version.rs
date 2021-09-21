#[derive(Clone, Debug, Serialize)]
pub struct Version {
    pub version: &'static str,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
