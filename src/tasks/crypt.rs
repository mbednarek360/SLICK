// ----------------------------------------------------------------
// pad or depad vector with zeroes to match key length
fn vec_pad(k: u64, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let mut out = v.clone();
    let l = v.len() as u64;
    if e {
        out.push(255);
        for _ in 0..(k - l) {
            out.push(0);
        }
    }
    else {
        while out[out.len() - 1] != 255 {
            out.pop();
        }
        out.pop();
    }
    return out;
}


// ----------------------------------------------------------------
// calculate position to find value for current byte
fn vec_position(k: u128, s: usize) -> usize {
    return (k % s as u128) as usize;
}


// ----------------------------------------------------------------
// shift byte value by current position
fn vec_shift(b: u8, n: usize, e: bool) -> u8 {
    let max = 256;
    let pos = n as u64 % max;
    if e {
        return (((b as u64 + pos) % max) ^ pos) as u8;
    }
    else {
        return (((b as u64 ^ pos) - pos) % max) as u8;
    }
}


// ----------------------------------------------------------------
// generate shuffled accending vector from key
pub fn gen_vec(k: u128, l: u64) -> Vec<usize> {
    let mut x = k - 1;
    let mut a: Vec<usize> = (0..(l as usize)).collect();
    let mut s = l as usize;
    while s > 0 {
        let p = vec_position(x, s);
        a.swap(p, s - 1);
        x /= s as u128;
        s -= 1;
    }
    return a;
}


// ----------------------------------------------------------------
// primary byte vector crypt function 
pub fn vec_crypt(k: u128, l: u64, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let a: Vec<u8>; 
    if e {
        a = vec_pad(l, v, e);
    } else {
        a = v.clone();
    }
    let p = gen_vec(k, l + 1);
    let mut f = Vec::with_capacity(l as usize + 1);
    unsafe { f.set_len(l as usize + 1); }
    let mut s: usize;
    let mut o: usize;
    for c in 0..(l as usize + 1) {
        s = p[c] + 1;
        o = p[l as usize - c] + 1;
        if e {
            f[c] = vec_shift(
            vec_shift(a[p[c]], s, e), o, e);
        }
        else {
            f[p[c]] = vec_shift(
            vec_shift(a[c], o, e), s, e);
        }
    }
    if e {
        return f;
    }
    else {
        return vec_pad(l, &f, e); 
    }
}
