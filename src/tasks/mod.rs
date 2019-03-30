mod crypt;
mod key;
mod file;

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