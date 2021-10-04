use sha2::{Digest, Sha256};

#[inline]
pub fn sha2(value: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(value);
    format!("{:x}", hash.finalize())
}
