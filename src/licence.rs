use sha2::{Digest, Sha256};

pub fn check(licence: u64) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(to_u8(licence));
    let hashed = hasher.finalize();
    for i in hashed[29..32].iter() {
        if *i == b'a' {
            continue;
        }
        return false;
    }
    return true;
}

fn to_u8(input: u64) -> Vec<u8> {
    let mut returnvec: Vec<u8> = Vec::new();
    for i in 0..8 {
        returnvec.push((input >> i * 8) as u8);
    }
    returnvec
}
