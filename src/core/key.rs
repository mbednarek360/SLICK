use super::conf::*;
use rand::RngCore;
use blake3;

// generate random or seeded key
pub fn generate(seed: Option<String>) -> KeyType {
    KeyType::from({
        match seed {
            Some(value) => derbytes(value),
            None => randbytes() 
        }
    })
}

// derive bytes from string with blake3
fn derbytes(seed: String) -> [u8; KEY_SIZE / 8] {
    let mut key = [0u8; KEY_SIZE / 8];
    blake3::derive_key(concat!(env!("CARGO_PKG_NAME"), " KEY"),
        seed.as_bytes(), &mut key);
    key
}

// generate random byte sequence
fn randbytes() -> [u8; KEY_SIZE / 8] {
    let mut seed = rand::thread_rng(); 
    let mut key = [0u8; KEY_SIZE / 8];
    seed.fill_bytes(&mut key);
    key
}

// derive nonce and seed from key
fn splitkey(key: &[u8]) -> ([u8; KEY_SIZE / 8],  [u8; KEY_SIZE / 8]) {
    let mut nonce = [0u8; KEY_SIZE / 8]; 
    let mut seed = [0u8; KEY_SIZE / 8]; 
    blake3::derive_key(concat!(env!("CARGO_PKG_NAME"),
        " NONCE"), key, &mut nonce);
    blake3::derive_key(concat!(env!("CARGO_PKG_NAME"), 
        " SEED"), key, &mut seed);
    (nonce, seed)
}
