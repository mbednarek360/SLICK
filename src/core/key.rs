use super::conf::*;
use rand::RngCore;
use rand;
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
    blake3::derive_key(env!("CARGO_PKG_NAME"),
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

