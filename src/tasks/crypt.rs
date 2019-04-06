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
    let mut x = k;
    let mut a: Vec<u8>;
    if e {
        a = vec_pad(l, v, e);
    } else {
        a = v.clone();
    }
    let mut f = Vec::new();
    for p in 0..l {
       let s = l - p;
       let b = vec_position(x, s);
       f.push(a[b]);
       a.remove(b);
       x /= s as u128;
    }
    if e {
        return f;
    }
    else {
        return vec_pad(l, &f, e); 
    }
}
