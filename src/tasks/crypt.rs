// ----------------------------------------------------------------
// pad or depad vector with zeroes to match key length
fn vec_pad(k: u64, v: &Vec<u8>, e: bool) -> Vec<u8> {
    let mut out = v.clone();
    let l = v.len() as u64;
    if k > l {
        if e {
            out.push(255);
            for i in 0..((k - l) - 1) {
                out.push(0);
            }
        }
        else {
            while out[out.len() - 1] != 255 {
                out.pop();
            }
            out.pop();
        }
    }
    return out;
}


// ----------------------------------------------------------------
// calculate position to find value for current byte
fn vec_position(k: u128, s: u64) -> usize {
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
pub fn gen_vec(k: u128, l: u64, e: bool) -> Vec<usize> {
    let mut x = k;
    let mut a: Vec<usize> = (0..(l as usize)).collect();
    let mut f = vec![0; l as usize];
    for p in 0..l {
        let s = l - p;
        let b = vec_position(x, s);
        if e {
            f[p as usize] = a[b];
        }
        else {
            f[a[b]] = p as usize;
        }
        a.remove(b);
        x /= s as u128;
    }
    return f;
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
    let p = gen_vec(k, l, e);
    let mut f = Vec::new();
    let mut s: usize;
    for c in 0..(l as usize) {
        if e {
            s = p[c] + 1;
        }
        else {
            s = c + 1;
        }
        f.push(vec_shift(a[p[c]], s, e));
    }
    if e {
        return f;
    }
    else {
        return vec_pad(l, &f, e); 
    }
}