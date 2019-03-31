use std::fs::File;
use std::io::Read;

// ----------------------------------------------------------------
// read file to u8 vector
pub fn read_file(s: &String) -> Vec<u8> {
    let mut f = File::open(s).unwrap();
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b);
    return b;
}