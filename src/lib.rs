mod utils;
use std::iter::{self, successors, Successors};

use crate::utils::*;

include!(concat!(env!("OUT_DIR"), "/sboxes.rs"));

fn compress(char: [u64; 8], state: &mut [u64; 3]) {
    compress_with_sbox(char, state, SBOXES)
}

fn hash(bytes: Vec<u8>) -> String{
    let mut state = START_VALUES;
    let message_len = (bytes.len() & 0xFF) as u64;

    let mut compress_chunk = |chunk: &[u8]| {
        let char: [u64; 8] = chunk
            .chunks_exact(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
        compress(char, &mut state);
    };

    let mut iter = bytes.chunks_exact(64);
    iter.by_ref().for_each(&mut compress_chunk);
    let mut remainder = iter.remainder().to_vec();
    remainder.push(0x01);
    remainder
        .extend(iter::repeat(0x00).take((56 - remainder.len() as i64).rem_euclid(64) as usize));
    remainder.append(&mut (message_len << 3).to_le_bytes().to_vec());
    remainder.chunks_exact(64).for_each(&mut compress_chunk);

    let state = state.map(|x| u64::from_le_bytes(x.to_be_bytes()));

    format!("{:016X}{:016X}{:016X}", state[0], state[1], state[2])
}

fn xrange(
    start: i64,
    end: i64,
    step: i64,
) -> std::iter::Successors<i64, impl FnMut(&i64) -> Option<i64>> {
    successors(
        if step > 0 && start < end || step < 0 && start > end {
            Some(start)
        } else {
            None
        },
        move |&i| {
            let next = i + step;
            if step > 0 && next < end || step < 0 && next > end {
                Some(next)
            } else {
                None
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input: Vec<u8> =
            String::from("")
                .into_bytes();
        assert_eq!(hash(input), "3293AC630C13F0245F92BBB1766E16167A4E58492DDE73F3");
    }

    #[test]
    fn test_chunks() {
        let block = [0x0123456789ABCDEF; 8];
        assert_eq!(block, readChunks(writeChunks(block)))
    }

    #[test]
    fn test_xrange() {
        let start: i64 = 0;
        let end: i64 = 10;
        let step: i64 = 2;
        let expected_output: Vec<i64> = vec![0, 2, 4, 6, 8];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = 0;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = -1;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );

        let start: i64 = 10;
        let end: i64 = -5;
        let step: i64 = -2;
        let expected_output: Vec<i64> = vec![10, 8, 6, 4, 2, 0, -2, -4];
        assert_eq!(
            xrange(start, end, step).collect::<Vec<i64>>(),
            expected_output
        );
    }
}
