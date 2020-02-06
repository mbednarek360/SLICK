use rand::Rng;
use base64;
use hex;


// ----------------------------------------------------------------
// return c! or max u128
pub fn max_key(c: usize) -> u128 {
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
pub fn gen_key(l: usize, r: u16) -> String {
    let k: u128 = rand::thread_rng().gen_range(1, max_key(l));
    let s = format!("{:0>4x}{:0>16X}{:0>32X}", r, l, k);
    return base64::encode(&hex::decode(s).unwrap());
}


// ----------------------------------------------------------------
// parses key string to length and seed
pub fn parse_key(k: &String) -> (u16, usize, u128) {
    let h = hex::encode(base64::decode(k).unwrap());
    let r = u16::from_str_radix(&h[0..4], 16).unwrap();
    let l = usize::from_str_radix(&h[4..20], 16).unwrap();
    let s = u128::from_str_radix(&h[20..52], 16).unwrap();
    return (r, l, s);
}
