use std::{env, fmt, path::Path};

#[cfg(any(target_os = "linux"))]
use dirs;

use nanoid;
use serde::Serialize;

macro_rules! mount_path {
    () => {
        "/fiscalidade/v1"
    };
    ($path:literal) => {
        concat!(mount_path!(), $path)
    };
}

macro_rules! json_ok {
    ($result:expr) => {
        rocket_contrib::json!({ "status": "ok", "result": $result })
    };
}

macro_rules! json_error {
    ($reason:expr) => {
        rocket_contrib::json!({ "status": "error", "reason": $reason })
    };
}

#[derive(Serialize)]
pub struct Version {
    pub major: u8,
    pub minor: u16,
    pub patch: u32,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Serialize)]
pub struct Info {
    pub name: String,
    pub long_name: String,
    pub version: Version,
    pub authors: String,
    pub arch: String,
    pub os: String,
}

impl Info {
    pub fn new() -> Self {
        use self::*;
        Self {
            name: env!("CARGO_PKG_NAME").into(),
            long_name: "Fiscalidade Server".into(),
            version: Version::new(),
            authors: env!("CARGO_PKG_AUTHORS").into(),
            arch: env::consts::ARCH.into(),
            os: env::consts::OS.into(),
        }
    }
}

impl fmt::Display for Info {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{} {} ({}-{})",
            self.name, self.version, self.arch, self.os
        )
    }
}

#[inline]
pub fn basename(ext: &str) -> String {
    #[cfg(target_os = "windows")]
    let path = env::current_exe().unwrap_or_default();
    #[cfg(any(target_os = "linux"))]
    let path = dirs::home_dir()
        .unwrap_or_default()
        .join(format!(".{}", Info::new().name));
    Path::new(&path).with_extension(ext).display().to_string()
}

#[inline]
pub fn generate_token() -> String {
    const TOKEN_CHARS: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    nanoid::nanoid!(22, &TOKEN_CHARS)
}
