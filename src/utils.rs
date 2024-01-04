use std::num::Wrapping;

type Chunk = u64;

pub const TIGER_PASSES: u32 = 3;

const WRAPPED_FF : Wrapping<u64> = Wrapping(0xFF);

const IS_LITTLE_ENDIAN : bool = cfg!(target_endian = "little");

pub const START_VALUES : [u64; 3] = [0x0123456789ABCDEF, 0xFEDCBA9876543210, 0xF096A5B4C3B2E187];

pub fn compress_with_sbox(char: [u64; 8], state: &mut [u64; 3], sboxes: [[u8; 8]; 1024]) {
    let (mut a, mut b, mut c) = (Wrapping(state[0]), Wrapping(state[1]), Wrapping(state[2]));
    let (aa, bb, cc) = (a, b, c);
    let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7) = (
        Wrapping(char[0]),
        Wrapping(char[1]),
        Wrapping(char[2]),
        Wrapping(char[3]),
        Wrapping(char[4]),
        Wrapping(char[5]),
        Wrapping(char[6]),
        Wrapping(char[7]),
    );

    let sb1 = &sboxes[0..256].try_into().unwrap();
    let sb2 = &sboxes[256..512].try_into().unwrap();
    let sb3 = &sboxes[512..768].try_into().unwrap();
    let sb4 = &sboxes[768..1024].try_into().unwrap();

    for pass_no in 0..TIGER_PASSES {
        if pass_no != 0 {
            x0 -= x7 ^ Wrapping(0xA5A5A5A5A5A5A5A5);
            x1 ^= x0;
            x2 += x1;
            x3 -= x2 ^ ((!x1) << 19);
            x4 ^= x3;
            x5 += x4;
            x6 -= x5 ^ ((!x4) >> 23);
            x7 ^= x6;
            x0 += x7;
            x1 -= x0 ^ ((!x7) << 19);
            x2 ^= x1;
            x3 += x2;
            x4 -= x3 ^ ((!x2) >> 23);
            x5 ^= x4;
            x6 += x5;
            x7 -= x6 ^ Wrapping(0x0123456789ABCDEF);
        }
        let mul = Wrapping(if pass_no == 0 { 5 } else { if pass_no == 1 { 7 } else { 9 } });
        round(&mut a, &mut b, &mut c, x0, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x1, mul, sb1, sb2, sb3, sb4);
        round(&mut c, &mut a, &mut b, x2, mul, sb1, sb2, sb3, sb4);
        round(&mut a, &mut b, &mut c, x3, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x4, mul, sb1, sb2, sb3, sb4);
        round(&mut c, &mut a, &mut b, x5, mul, sb1, sb2, sb3, sb4);
        round(&mut a, &mut b, &mut c, x6, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x7, mul, sb1, sb2, sb3, sb4);

        (a, b, c) = (c, a, b);
    }

    a ^= aa;
    b -= bb;
    c += cc;

    (state[0], state[1], state[2]) = (a.0, b.0, c.0);
}

fn round(
    a: &mut Wrapping<u64>,
    b: &mut Wrapping<u64>,
    c: &mut Wrapping<u64>,
    x: Wrapping<u64>,
    mul: Wrapping<u64>,
    sb1: &[[u8; 8]; 256],
    sb2: &[[u8; 8]; 256],
    sb3: &[[u8; 8]; 256],
    sb4: &[[u8; 8]; 256],
) {
    *c ^= x;
    *a -= read_u64(&sb1[((*c >> (0 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb2[((*c >> (2 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb3[((*c >> (4 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb4[((*c >> (6 * 8)) & WRAPPED_FF).0 as usize]);
    *b += read_u64(&sb4[((*c >> (1 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb3[((*c >> (3 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb2[((*c >> (5 * 8)) & WRAPPED_FF).0 as usize])
        ^ read_u64(&sb1[((*c >> (7 * 8)) & WRAPPED_FF).0 as usize]);
    *b *= mul;
}

fn read_u64(bytes: &[u8]) -> u64 {
    <byteorder::LittleEndian as byteorder::ByteOrder>::read_u64(bytes)
}

pub fn readChunks(bytes: [u8;64]) -> [u64; 8] {
    (0..8).map(|i| {
        let mut slice = if !IS_LITTLE_ENDIAN { bytes[63 - (i+1)*8.. 63 - i*8].to_vec() } else { bytes[i*8..(i+1)*8].to_vec() };
        if !IS_LITTLE_ENDIAN { slice.reverse() };
        <byteorder::LittleEndian as byteorder::ByteOrder>::read_u64(&slice)
    })
    .collect::<Vec<u64>>().try_into().unwrap()
}