// parameters
pub type KeyType = primitive_types::U256;
pub type BlockType = u8;
pub const KEY_SIZE: usize = 256;
#[cfg(not(test))] 
pub const BLOCK_SIZE: usize = 57;

// test parameters
#[cfg(test)]
pub const BLOCK_SIZE: usize = 7;

// for collision testing
#[cfg(test)]
pub const PERM_COUNT: usize = gen_fact();
#[cfg(test)]
const fn gen_fact() -> usize {
    let mut product = 1;
    let mut index = 2;
    while index <= BLOCK_SIZE {
        product *= index;
        index += 1;
    }
    product
}

// for permutation generation
pub const PERM_INIT: [BlockType; BLOCK_SIZE] = gen_array();
const fn gen_array() -> [BlockType; BLOCK_SIZE] {
    let mut output = [0 as BlockType; BLOCK_SIZE];
    let mut index: usize = 0;
    while index < BLOCK_SIZE {
        output[index] = index as BlockType;
        index += 1;
    }
    output
}
