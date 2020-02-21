use std::env;

use nanoid;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_LONG_NAME: &'static str = "Fiscalidade Server";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const APP_ARCH: &'static str = env::consts::ARCH;
pub const APP_OS: &'static str = env::consts::OS;

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
