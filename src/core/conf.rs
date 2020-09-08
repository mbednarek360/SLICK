// constants
pub type KeyType = ethereum_types::U256;
pub type BlockType = u8;
pub const KEY_SIZE: usize = 256;
pub const BLOCK_SIZE: usize = 57;
pub const PERM_INIT: [BlockType; BLOCK_SIZE] = gen_array();

// for permutation init 
const fn gen_array() -> [BlockType; BLOCK_SIZE] {
    let mut output = [0 as BlockType; BLOCK_SIZE];
    let mut index: usize = 0;
    while index < BLOCK_SIZE {
        output[index] = index as BlockType;
        index += 1;
    }
    output
}
