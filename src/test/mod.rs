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
    // create random input of small size
    // go through all keys iteratively
    // fail if any two outputs match
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
