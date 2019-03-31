mod crypt;
mod key;
mod file;

// ----------------------------------------------------------------
pub fn file_crypt(k: &String, f: &String, e: bool) {
    let key = key::parse_key(&k);
    let data = file::read_file(&f);
    let enc = crypt::vec_crypt(key.1, key.0, &data);

    println!("{:?}", enc);

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