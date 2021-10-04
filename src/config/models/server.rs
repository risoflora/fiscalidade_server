use std::net::IpAddr;

#[inline]
fn default_host() -> IpAddr {
    "0.0.0.0".parse().unwrap()
}

#[inline]
fn default_port() -> u16 {
    8080
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServidorConfiguration {
    #[serde(default = "default_host")]
    pub host: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServidorConfiguration {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}
