use sha2::{Sha256, Digest};

pub fn check(licence: String) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(licence.as_bytes());
    let hashed = hasher.finalize();
    for i in hashed[250..256].iter() {
        if *i == b'a' {
            continue
        }
        return false;
    }
    return true;
}