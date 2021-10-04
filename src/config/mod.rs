use std::{
    collections::HashMap,
    fs::read_to_string,
    io::Result,
    path::{Path, PathBuf},
};

use crate::home::home_dir;

use self::models::{deployment::DeploymentConfiguration, server::ServidorConfiguration};

pub mod models;

#[inline]
pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|p| p.join("config.toml"))
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub server: ServidorConfiguration,
    #[serde(default)]
    pub deployments: Vec<DeploymentConfiguration>,
}

pub type Deployments = HashMap<String, DeploymentConfiguration>;

impl Configuration {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = read_to_string(path).unwrap_or_default();
        Ok(toml::from_str(&config)?)
    }

    pub fn deployments(&self) -> Deployments {
        self.deployments
            .clone()
            .into_iter()
            .map(|deployment| (deployment.token(), deployment))
            .collect()
    }
}
