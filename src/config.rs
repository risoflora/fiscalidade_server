use std::{
    fmt,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use dirs;
use dotenv;

use super::AppProps;

pub struct Config {
    pub port: u16,
    pub database: String,
    #[cfg(not(feature = "embed_webservices"))]
    pub webservices: String,
    pub silent: bool,
}

impl Config {
    fn prepare() -> io::Result<()> {
        fs::create_dir_all(Self::dir())?;
        let filename = Self::filename();
        if !Path::new(&filename).exists() {
            let mut file = File::create(&filename)?;
            file.write_all(Self::default().to_string().as_bytes())?;
        }
        Ok(())
    }

    pub fn dir() -> String {
        Path::new(
            &dirs::config_dir()
                .unwrap_or_default()
                .into_os_string()
                .into_string()
                .unwrap_or_default(),
        )
        .join("fiscalidade")
        .display()
        .to_string()
    }

    pub fn filename() -> String {
        Path::new(&Self::dir())
            .join(env!("CARGO_PKG_NAME"))
            .with_extension("conf")
            .display()
            .to_string()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        Self::prepare()?;
        dotenv::from_filename(path)?;
        Ok(Config {
            port: dotenv::var("port")?.parse::<u16>()?,
            database: dotenv::var("database")?,
            #[cfg(not(feature = "embed_webservices"))]
            webservices: dotenv::var("webservices")?,
            silent: dotenv::var("silent")?.parse::<bool>()?,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            database: String::from("postgres://postgres:postgres@localhost/postgres"),
            #[cfg(not(feature = "embed_webservices"))]
            webservices: Default::default(),
            silent: false,
        }
    }
}

impl fmt::Display for Config {
    #[cfg(feature = "embed_webservices")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "port={}\ndatabase={}\nsilent={}",
            self.port, self.database, self.silent,
        )
    }
    #[cfg(not(feature = "embed_webservices"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "port={}\ndatabase={}\nwebservices={}\nsilent={}",
            self.port, self.database, self.webservices, self.silent,
        )
    }
}

impl From<Config> for AppProps {
    fn from(opts: Config) -> Self {
        Self {
            port: opts.port,
            database: opts.database,
            #[cfg(not(feature = "embed_webservices"))]
            webservices: opts.webservices,
            install: false,
            uninstall: false,
            silent: opts.silent,
        }
    }
}
