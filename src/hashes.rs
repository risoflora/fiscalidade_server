use std::{fs::File, io, path::Path};

use sha2::{Digest, Sha256};

#[inline]
pub fn sha256(value: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(value);
    let hash = sha256.finalize();
    format!("{:x}", hash)
}

#[inline]
pub fn sha256sum<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    let hash = sha256.finalize();
    Ok(format!("{:x}", hash))
}
