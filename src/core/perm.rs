use super::conf::*;

// convert key to permutation array
pub fn gen_perm(key: KeyType) -> [BlockType; BLOCK_SIZE] {
    let mut rem = key;
    let mut perm = PERM_INIT;
    for radix in (0..BLOCK_SIZE).rev() {
        let calc = rem.div_mod(KeyType::from(radix + 1));            
        let index = calc.1;
        perm.swap(index.as_usize(), radix);        
        rem = calc.0;
    }
    perm
}
