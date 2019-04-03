// ----------------------------------------------------------------
// pad or depad vector with zeroes to match key length
fn vec_pad(k: u64, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let mut out = v.clone();
    if e {
        let l = v.len() as u64;
        for i in 0..(k - l) {
            out.push(0);
        }
        out.push(255);
    }
    else {
        while v[0] != 255 {
            out.pop();
        }
    }
    return out;
}


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
fn vec_shift(b: u8, n: u64, e: bool) -> u8 {
    let max = 256;
    let pos = n % max;
    if e {
        return (((b as u64 + pos) % max) ^ pos) as u8;
    }
    else {
        return (((b as u64 ^ pos) - pos) % max) as u8;
    }
}

// ----------------------------------------------------------------
// primary byte vector crypt function 
pub fn vec_crypt(k: u128, l: u64, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let a = vec_pad(l, v, e);
    let mut f = Vec::new();
    let c = vec_const(k, l);
    for i in 0..l {
        let p = vec_position(i, c, l);
        let s;
        if e {
            s = p + 1;
        }
        else {
            s = i + 1;
        }
        let b = vec_shift(a[p as usize], s, e);
        f.push(b);
    }
    return f;
}
