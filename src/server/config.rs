use std::net::IpAddr;

fn default_host() -> IpAddr {
    "0.0.0.0".parse().unwrap()
}

fn default_port() -> u16 {
    8080
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfiguration {
    #[serde(default = "default_host")]
    pub host: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}
