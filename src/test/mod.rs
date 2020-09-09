use super::core::conf::*;
use super::core::perm;

// check encryption / decryption results in original
#[test]
fn integrity() {
    // create random input and key
    // encrypt
    // decrypt with correct key
    // fail if different than original
    // decrypt with incorrect key
    // fail if identical to original
}

#[test]
fn collision() {
    let mut perms: [[u8; BLOCK_SIZE]; PERM_COUNT] = [[0; BLOCK_SIZE]; PERM_COUNT]; 
    for key in 0..PERM_COUNT {
        let perm = perm::gen_perm(KeyType::from(key));
        assert!(!perms.contains(&perm));
        perms[key] = perm;
    }
}

#[test]
fn performance() {
    // create random input and key
    // start timer
    // encrypt 
    // decrypt
    // stop timer
    // sum times for x runs
}
