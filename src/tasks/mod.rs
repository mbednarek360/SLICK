mod crypt;
mod key;
mod file;
use time::precise_time_ns;


// ----------------------------------------------------------------
// primary file encryption function
pub fn file_crypt(k: &String, f: &String, e: bool) {
    let (round, size, seed) = key::parse_key(&k);
    let data = file::read_file(&f);
    let enc = crypt::vec_crypt(seed, size, round, &data, e);
    file::write_file(&f, &enc);
}


// ----------------------------------------------------------------
// generate and display key
pub fn gen_key(s: &String, c:&String) {
    let l: usize = s.parse().unwrap();
    let r: u16 = c.parse().unwrap();
    let k = key::gen_key(l, r);
    println!("{}-byte key:\n", l);
    println!("{}", &k);
}


// ----------------------------------------------------------------
// run vector generation for all lengths upto given
pub fn test(f: &String, s: &String) {
    let b: usize = f.parse().unwrap();
    let l: usize = s.parse().unwrap();
    let r: u16 = s.parse().unwrap();
    for i in b..l + 1 {
        let v = vec![0; i];
        let start = precise_time_ns();
        let _ = crypt::vec_crypt(i as u128, i, r, &v, true);
        let end = precise_time_ns();
        println!("{}, {}", i, end - start);
    }
}


// ----------------------------------------------------------------
// display available commands
pub fn help() {
    println!("Available commands:
    
    -e <key> <file>    | Encryption command.
    -d <key> <file>    | Decryption command.
    -k <size> <rounds> | Key generation command.
    -t <size>          | Speed test command.

SLICK v0.1.3
https://github.com/mbednarek360/SLICK");
}


// ----------------------------------------------------------------
// error for invalid command
pub fn error() {
    println!("Invalid command. See -h for a list of commands.");
}
