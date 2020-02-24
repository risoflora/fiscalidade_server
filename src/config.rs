use std::path::Path;

use crate::anyhow;
use dirs;
use dotenv;

use super::AppProps;

pub struct Config {
    pub port: u16,
    pub database: String,
    #[cfg(not(feature = "embed_webservices"))]
    pub webservices: String,
    pub migrations: bool,
    pub silent: bool,
}

impl Config {
    pub fn dir() -> String {
        format!(
            "{}/{}",
            dirs::config_dir()
                .unwrap_or_default()
                .into_os_string()
                .into_string()
                .unwrap_or_default(),
            env!("CARGO_PKG_NAME")
        )
    }

    pub fn filename() -> String {
        format!("{}/{}.conf", Self::dir(), env!("CARGO_PKG_NAME"))
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        dotenv::from_filename(path)?;
        Ok(Config {
            port: dotenv::var("port")?.parse::<u16>()?,
            database: dotenv::var("database")?,
            #[cfg(not(feature = "embed_webservices"))]
            webservices: dotenv::var("webservices")?,
            migrations: dotenv::var("migrations")?.parse::<bool>()?,
            silent: dotenv::var("silent")?.parse::<bool>()?,
        })
    }
}

impl From<Config> for AppProps {
    fn from(opts: Config) -> Self {
        Self {
            port: opts.port,
            database: opts.database,
            #[cfg(not(feature = "embed_webservices"))]
            webservices: opts.webservices,
            migrations: opts.migrations,
            silent: opts.silent,
        }
    }
}
