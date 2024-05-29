use ripemd::{Ripemd160, Digest};

pub fn ripemd160(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();

    hasher.update(bytes);
    hasher.finalize().to_vec()
}

