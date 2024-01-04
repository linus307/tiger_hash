mod utils;
use std::iter::{successors, Successors};

use crate::utils::*;

include!(concat!(env!("OUT_DIR"), "/sboxes.rs"));

fn compress(char: [u64; 8], state: &mut [u64; 3]) {
    compress_with_sbox(char, state, SBOXES)
}

fn hash(chars: Vec<u8>){
    let mut temp: [u8; 64] = [0; 64];
    let mut values = START_VALUES;
    let mut j = 0;
    for i in (0..=chars.len()-64).step_by(64){
        let chunks = readChunks(chars[i..i+64].try_into().unwrap());
        compress(chunks, &mut values);
        j = i;
    }
    for i in (0..j - chars.len() + 64){
        if i + chars.len() - 63 < chars.len() {
            temp[i] = chars[i + chars.len() - 63];
        }
        else if i + chars.len() - 63 == chars.len() {
            temp[i] = 1;
        }
        else {
            temp[i] = 0;
        }
    }
    compress(readChunks(temp), &mut values);
}

fn xrange(start: i64, end: i64, step: i64) -> std::iter::Successors<i64, impl FnMut(&i64) -> Option<i64>> {
    successors(if step > 0 && start < end || step < 0 && start > end {Some(start)} else {None},
    move |&i| {
        let next = i + step;
        if step > 0 && next < end || step < 0 && next > end {Some(next)} else {None}
    } )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xrange() {
        let start: i64 = 0;
        let end: i64 = 10;
        let step: i64 = 2;
        let expected_output: Vec<i64> = vec![0, 2, 4, 6, 8];
        assert_eq!(xrange(start, end, step).collect::<Vec<i64>>(), expected_output);

        let start: i64 = 10;
        let end: i64 = 0;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2];
        assert_eq!(xrange(start, end, step).collect::<Vec<i64>>(), expected_output);

        let start: i64 = 10;
        let end: i64 = -1;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0];
        assert_eq!(xrange(start, end, step).collect::<Vec<i64>>(), expected_output);

        let start: i64 = 10;
        let end: i64 = -5;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0, -2, -4];
        assert_eq!(xrange(start, end, step).collect::<Vec<i64>>(), expected_output);
    }
    
}