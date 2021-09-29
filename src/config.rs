use std::{
    fs::read_to_string,
    io::Result,
    path::{Path, PathBuf},
};

use thiserror::Error;
use toml::de::Error as TomlError;

use crate::{
    cert::config::CertificateConfiguration, home::home_dir, server::config::ServerConfiguration,
};

#[inline]
pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|p| p.join("config.toml"))
}

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error(transparent)]
    Toml(#[from] TomlError),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub server: ServerConfiguration,
    #[serde(rename = "certificate")]
    pub certificates: Option<Vec<CertificateConfiguration>>,
}

impl Configuration {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = read_to_string(path).unwrap_or_default();
        Ok(toml::from_str(&config)?)
    }
}
