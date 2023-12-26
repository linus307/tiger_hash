use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use rayon::prelude::*;
use byteorder::{ByteOrder, LittleEndian};

include!("src/utils.rs");

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sboxes.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Replace this with your actual computation
    let sboxes: [[u8; 8]; 1024] = gen_sboxes();

    write!(f, "pub const SBOXES: [[u8; 8]; 1024] = {:?};", sboxes).unwrap();
}

fn gen_sboxes() -> [[u8; 8]; 1024] {
    let binding = String::from("Tiger - A Fast New Hash Function, by Ross Anderson and Eli Biham");
    let str = binding.as_bytes();
    let state: [u64; 3] = [0x0123456789ABCDEF, 0xFEDCBA9876543210, 0xF096A5B4C3B2E187];

    let temp: [u64; 8] = (0..8).map(|i| {
            let bytes: [u8; 8] = (i*8..i*8+8).map(|i: u8|{
                str[i.to_le() as usize] as u8
            })
            .collect::<Vec<u8>>().try_into().unwrap();
            LittleEndian::read_u64(&bytes)
        })
        .collect::<Vec<u64>>().try_into().unwrap();

    let mut sboxes: [[u8; 8]; 1024] = (0..1024)
        .into_par_iter()
        .map(|i| {
            let bytes: Vec<u8> = (0..8).map(|_| (i & 0xFF) as u8).collect();
            bytes.try_into().unwrap()
        })
        .collect::<Vec<[u8; 8]>>().try_into().unwrap();

    let mut abc = 2;

    for pass in 0..5 {
        for i in 0..256 {
            for sb in (0..1024).step_by(256) {
                abc += 1;
                if abc >= 3 { abc = 0; compress_with_sbox(temp, state, sboxes); }
                sboxes[sb + i][pass] = temp[pass] as u8;
                for col in 0..8 {
                    let val: u8 = sboxes[sb + i][col];
                    sboxes[sb + i][col] = sboxes[sb + (state[abc].to_le_bytes()[col] as usize)][col];
                    sboxes[sb + (state[abc].to_le_bytes()[col] as usize)][col] = val;
                }
            }
        }
    }

    sboxes
}
