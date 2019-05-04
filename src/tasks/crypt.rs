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
// shifts values in vector given subdivided key vector
fn vec_shift(l: usize, v: &Vec<u8>, i: &Vec<usize>, e: bool) -> Vec<u8> {

}


// ----------------------------------------------------------------
// substitutes values in a vector using shuffled bytes made with an index vector
fn vec_sub(l: usize, v: &Vec<u8>, i: &Vec<usize>, e: bool) -> Vec<u8> {

}


// ----------------------------------------------------------------
// generates a permutation of a vector given an index vector
fn vec_permute(l: usize, v: &Vec<u8>, i: &Vec<usize>, e: bool) -> Vec<u8> {

}


// ----------------------------------------------------------------
// subdivides key into vector to be used for index and shift generation
fn key_div(k: u128, l: usize) -> Vec<u128> {

} 


// ----------------------------------------------------------------
// generate shuffled accending index vector from subdivided key vector
fn gen_index(l: usize, k: &Vec<u128>) -> Vec<usize> {

}


// ----------------------------------------------------------------
// primary byte vector crypt function 
pub fn vec_crypt(k: u128, l: usize, r: u16, v: &Vec<u8>, e: bool) -> Vec<u8> {

}