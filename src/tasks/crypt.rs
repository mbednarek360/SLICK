// ----------------------------------------------------------------
// find vector constant
fn vec_const(k: u128, l: u64) -> u64 {
    return ((k / l as u128) + (k % l as u128)) as u64;
}


// ----------------------------------------------------------------
// calculate position to find value for current byte
fn vec_position(i: u64, c: u64, l: u64) -> u64 {
    return (c - i) % l;
}


// ----------------------------------------------------------------
// shift byte value by current position
fn vec_shift(b: u8, n: u64) -> u8 {
    let max = 256;
    return ((b as u64 + (n % max)) % max) as u8;
}


// ----------------------------------------------------------------
// primary byte vector crypt function
pub fn vec_crypt(k: u128, l: u64, a: &Vec<u8>) -> Vec<u8> {
    let mut f = Vec::new();
    let c = vec_const(k, l);
    for i in 0..l {
        let p = vec_position(i, c, l);
        let b = vec_shift(a[p as usize], p + 1);
        f.push(b);
    }
    return f;
}