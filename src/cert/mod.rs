pub mod config;
pub mod store;

#[derive(Clone, Debug, Deserialize)]
pub struct Certificate {
    pub path: String,
    pub password: String,
}
