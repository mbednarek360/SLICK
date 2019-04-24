mod crypt;
mod key;
mod file;
use time::precise_time_ns;


// ----------------------------------------------------------------
// primary file encryption function
pub fn file_crypt(k: &String, f: &String, e: bool) {
    let key = key::parse_key(&k);
    let data = file::read_file(&f);
    let enc = crypt::vec_crypt(key.1, key.0, &data, e);
    file::write_file(&f, &enc);
}


// ----------------------------------------------------------------
// generate and display key
pub fn gen_key(s: &String) {
    let l: u64 = s.parse().unwrap();
    let k = key::gen_key(l);
    println!("{}-byte key:\n", l);
    println!("{}", k);
}


// ----------------------------------------------------------------
// prints every original index vector for given length
pub fn permute(s: &String) {
    let l: u64 = s.parse().unwrap();
    for i in 1..key::max_key(l) {
        println!("{:?}", crypt::gen_vec(i, l));
    }
}


// ----------------------------------------------------------------
// run vector generation for all lengths upto given
pub fn test(f: &String, s: &String) {
    let b: u64 = f.parse().unwrap();
    let l: u64 = s.parse().unwrap();
    for i in b..l + 1 {
        let v = vec![0; i as usize];
        let start = precise_time_ns();
        // let _ = crypt::gen_vec(i as u128, i);
        let _ = crypt::vec_crypt(i as u128, i, &v, true);
        let end = precise_time_ns();
        println!("{}, {}", i, end - start);
    }
}


// ----------------------------------------------------------------
// display available commands
pub fn help() {
    println!("Available commands:
    
    -e <key> <file> | Encryption command.
    -d <key> <file> | Decryption command.
    -k <size>       | Key generation command.
    -p <size>       | Permutation calculation command.
    -t <size>       | Speed test command.

SLICK v0.1.2
https://github.com/mbednarek360/SLICK");
}


// ----------------------------------------------------------------
// error for invalid command
pub fn error() {
    println!("Invalid command. See -h for a list of commands.");
}
