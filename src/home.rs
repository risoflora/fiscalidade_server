use std::path::PathBuf;

use dirs;

use crate::consts::APP_NAME;

#[inline]
pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(format!(".{}", APP_NAME)))
}
