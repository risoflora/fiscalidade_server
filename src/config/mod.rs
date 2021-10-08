use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use crate::home::home_dir;

use self::models::{deployment::DeploymentConfiguration, server::ServerConfiguration};

pub mod models;

#[inline]
pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|p| p.join("config.toml"))
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename = "servidor", default)]
    pub server: ServerConfiguration,
    #[serde(rename = "implantacao", default)]
    pub deployments: Vec<DeploymentConfiguration>,
}

pub type Deployments = HashMap<String, DeploymentConfiguration>;

impl Configuration {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let config = fs::read_to_string(path).unwrap_or_default();
        Ok(toml::from_str(&config)?)
    }

    pub fn deployments(&self) -> crate::Result<Deployments> {
        let mut deployments = Deployments::new();
        for deployment in self.deployments.clone() {
            deployments.insert(deployment.token()?, deployment.clone());
        }
        Ok(deployments)
    }
}
