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
// error for invalid command
pub fn error() {
    println!("Invalid command. See -h for a list of commands.");
}

// ----------------------------------------------------------------
// run vector generation for all lengths upto given
pub fn test(s: &String) {
    let l: u64 = s.parse().unwrap();
    for i in 1..l {
        let start = precise_time_ns();
        let x = crypt::gen_vec(i as u128, i, true);
        let end = precise_time_ns();
        println!("{}", end - start);
    }
}