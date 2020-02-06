// ----------------------------------------------------------------
// declare over byte value
const MAX_BYTE: usize = 256;


// ----------------------------------------------------------------
// pad or depad vector with zeroes to match key length
fn vec_pad(k: usize, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let mut out = v.clone();
    let l = v.len();
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
// shifts values in vector given index vector
fn vec_shift(l: usize, v: &Vec<u8>, i: &Vec<usize>, e: bool) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(l);
    unsafe { out.set_len(l); }
    if e {
        for c in 0..l {
                out[c] = (((v[c] as usize + i[c]) ^ i[c]) % MAX_BYTE) as u8;
        }
    }
    else {
        for c in 0..l {
                out[c] = (((v[c] as usize ^ i[c]) % MAX_BYTE) - i[c]) as u8;
        }
    }
    return out;
}


// ----------------------------------------------------------------
// substitutes values in a vector using shuffled bytes made with an index vector
fn vec_sub(k: u128, l: usize, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let i = gen_index(MAX_BYTE, k);
    let rng: Vec<u8> = (0..=(MAX_BYTE - 1) as u8).collect();
    let sub_vec = vec_permute(MAX_BYTE, &rng, &i, e);
    let mut out: Vec<u8> = Vec::with_capacity(l);
    unsafe { out.set_len(l); }
    for c in 0..l {
        out[c] = sub_vec[v[c] as usize];
    }
    return out;
}


// ----------------------------------------------------------------
// generates a permutation of a vector given an index vector
fn vec_permute(l: usize, v: &Vec<u8>, i: &Vec<usize>, e: bool) -> Vec<u8> {
    let mut f = Vec::with_capacity(l);
    unsafe { f.set_len(l); }
    for c in 0..l {
        if e {
            f[c] = v[i[c]];
        }
        else {
            f[i[c]] = v[c];
        }
    }
    return f;
}


// ----------------------------------------------------------------
// generate shuffled accending index vector from subdivided key vector
pub fn gen_index(l: usize, k: u128) -> Vec<usize> {
    let mut x = k - 1;
    let mut s = l; 
    let mut v: Vec<usize> = (0..l).collect();
    while s > 0 {
        let p = (x % s as u128) as usize;
        v.swap(p, s - 1);
        x /= s as u128;
        s -= 1;
    }
    return v;
}


// ----------------------------------------------------------------
// primary byte vector crypt function 
pub fn vec_crypt(k: u128, l: usize, r: u16, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let s = l + 1;
    let mut x = k;
    let mut a: Vec<u8>; 
    let mut i: Vec<usize>;
    if e {
        a = vec_pad(l, v, e);
        for c in 0..r {
            i = gen_index(s, k);
            a = vec_sub(k, s, &a, e);
            a = vec_permute(s, &a, &i, e);
            a = vec_shift(s, &a, &i, e);
            x = c as u128 ^ x;
        }
    }
    else {
        a = v.clone();
        for c in 0..r {
            i = gen_index(s, k);
            a = vec_shift(s, &a, &i, e);
            a = vec_permute(s, &a, &i, e);
            a = vec_sub(k, s, &a, e);
            x = c as u128 ^ x;  
        }
        a = vec_pad(l, &a, e); 
    }
    return a;
}
