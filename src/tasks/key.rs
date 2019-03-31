use rand::Rng;

// ----------------------------------------------------------------
// return c! or max u128
fn max_key(c: u64) -> u128 {
    let mut i = c as u128;
    let mut outp: u128 = 1;
    while i > 0 {
        match outp.checked_mul(i) {
            Some(v) => {
                outp = v;
                i -= 1;
            }
            None => {
                return u128::max_value();
            }
        }
    }
    return outp;
}


// ----------------------------------------------------------------
// generate key packaged with length and seed
pub fn gen_key(l: u64) -> String {
    let k: u128 = rand::thread_rng().gen_range(1, max_key(l));
    return format!("{:X}x{:X}", l, k);
}


// ----------------------------------------------------------------
// parses key string to length and seed
pub fn parse_key(k: &String) -> (u64, u128) {
    let v: Vec<&str> = k.split("x").collect();
    let l = u64::from_str_radix(v[0], 16).unwrap();
    let s = u128::from_str_radix(v[1], 16).unwrap();
    return (l, s);
}