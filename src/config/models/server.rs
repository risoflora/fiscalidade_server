use std::net::IpAddr;

#[inline]
fn default_host() -> IpAddr {
    "0.0.0.0".parse().unwrap()
}

#[inline]
fn default_post() -> u16 {
    3223
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfiguration {
    #[serde(rename = "endereco", default = "default_host")]
    pub host: IpAddr,
    #[serde(rename = "porta", default = "default_post")]
    pub port: u16,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_post(),
        }
    }
}
