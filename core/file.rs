use std::fs::File;
use std::io::Read;
use std::io::Write;


// ----------------------------------------------------------------
// read file to u8 vector
pub fn read_file(s: &String) -> Vec<u8> {
    let mut f = File::open(s).unwrap();
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b);
    return b;
}


// ----------------------------------------------------------------
// write u8 vector to file
pub fn write_file(s: &String, v: &Vec<u8>) {
    let mut f = File::create(s).unwrap();
    let mut d = v;
    f.write_all(&mut d);
}
