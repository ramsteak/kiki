use sha2::{Sha256, Digest};

pub fn hash_key(key: Option<&String>) -> u64{
    let key = match key {
        Some(key) => key.as_str(),
        None => ""
    };
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let res = hasher.finalize();

    u64::from_be_bytes(res[0..8].try_into().unwrap())
}