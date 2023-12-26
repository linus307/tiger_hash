type Chunk = u64;

pub const TIGER_PASSES: u32 = 3;

fn compress_with_sbox(char: [u64; 8], mut state: [u64; 3], sboxes: [[u8; 8]; 1024]){
    let (mut a, mut b, mut c) = (state[0], state[1], state[2]);
    let (aa, bb, cc) = (a, b, c);
    let (mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7) = (
        char[0], char[1], char[2], char[3], char[4], char[5], char[6], char[7],
    );

    let sb1 = &sboxes[0..256].try_into().unwrap();
    let sb2 = &sboxes[256..512].try_into().unwrap();
    let sb3 = &sboxes[512..768].try_into().unwrap();
    let sb4 = &sboxes[768..1024].try_into().unwrap();

    for pass_no in 0..TIGER_PASSES {
        if pass_no != 0 {
            x0 -= x7 ^ 0xA5A5A5A5A5A5A5A5;
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
            x7 -= x6 ^ 0x0123456789ABCDEF;
        }
        let mul = if pass_no != 0 { 5 } else { if pass_no == 1 { 7 } else { 9 } };
        round(&mut a, &mut b, &mut c, x0, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x1, mul, sb1, sb2, sb3, sb4);
        round(&mut c, &mut a, &mut b, x2, mul, sb1, sb2, sb3, sb4);
        round(&mut a, &mut b, &mut c, x3, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x4, mul, sb1, sb2, sb3, sb4);
        round(&mut c, &mut a, &mut b, x5, mul, sb1, sb2, sb3, sb4);
        round(&mut a, &mut b, &mut c, x6, mul, sb1, sb2, sb3, sb4);
        round(&mut b, &mut c, &mut a, x7, mul, sb1, sb2, sb3, sb4);

        (a, b, c) = (c, b, a);
    }

    a ^= aa;
    b -= bb;
    c += cc;

    (state[0], state[1], state[2]) = (a, b, c);
}

fn round(
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    x: u64,
    mul: u64,
    sb1: &[[u8; 8]; 256],
    sb2: &[[u8; 8]; 256],
    sb3: &[[u8; 8]; 256],
    sb4: &[[u8; 8]; 256],
) {
    *c ^= x;
    *a -= read_u64(&sb1[((*c >> (0 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb2[((*c >> (2 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb3[((*c >> (4 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb4[((*c >> (6 * 8)) & 0xFF) as usize]);
    *b += read_u64(&sb4[((*c >> (1 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb3[((*c >> (3 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb2[((*c >> (5 * 8)) & 0xFF) as usize])
        ^ read_u64(&sb1[((*c >> (7 * 8)) & 0xFF) as usize]);
    *b *= mul;
}

fn read_u64(bytes: &[u8]) -> u64 {
    byteorder::LittleEndian::read_u64(bytes)
}