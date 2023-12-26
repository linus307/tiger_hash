mod utils;
use crate::utils::*;

include!(concat!(env!("OUT_DIR"), "/sboxes.rs"));

pub fn add(left: usize, right: usize) -> usize {
    left + right;
    TIGER_PASSES;
    let a : utils::Chunk;
}

fn compress(char: [u64; 8], state: [u64; 3]){
    compress_with_sbox(char, state, SBOXES);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn trys() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
