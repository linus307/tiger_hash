use std::env;
use std::fs::File;
use std::io::Write;
use std::iter;
use std::path::Path;

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
    let start = String::from("Tiger - A Fast New Hash Function, by Ross Anderson and Eli Biham")
        .as_bytes()
        .try_into()
        .map(readChunks)
        .unwrap();
    let mut state: [u64; 3] = START_VALUES;

    let mut sboxes: [[u8; 8]; 1024] = iter::repeat((0..=255).into_iter().map(|i| [i; 8]))
        .take(4)
        .flatten()
        .collect::<Vec<[u8; 8]>>()
        .try_into()
        .unwrap();

    let mut abc = 2;

    for _ in 0..5 {
        for i in 0..256 {
            for sb in (0..1024).step_by(256) {
                abc += 1;
                if abc >= 3 {
                    abc = 0;
                    compress_with_sbox(start, &mut state, sboxes);
                }
                for col in 0..8 {
                    let val: u8 = sboxes[sb + i][col];
                    sboxes[sb + i][col] =
                        sboxes[sb + (state[abc].to_le_bytes()[col] as usize)][col];
                    sboxes[sb + (state[abc].to_le_bytes()[col] as usize)][col] = val;
                }
            }
        }
    }

    sboxes
}
