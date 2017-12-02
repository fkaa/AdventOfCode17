extern crate stdsimd;

fn main() {
    use std::io::BufRead;
    use std::str::FromStr;

    use stdsimd::vendor::*;
    use stdsimd::simd::*;

    let stdin = ::std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    let mut arr = [[0i32; 4]; 16];
    let mut row = 0;
    let mut checksums = i32x4::splat(0);

    while let Ok(n) = stdin.read_line(&mut buf) {
        if n == 0 { break; }
        buf.split_whitespace().map(|s| i32::from_str(&s).unwrap()).enumerate().for_each(|(idx, n)| arr[idx][row] = n);
        buf.clear();
        row += 1;

        if row == 4 {
            let columns = [
                i32x4::load(&arr[0], 0),
                i32x4::load(&arr[1], 0),
                i32x4::load(&arr[2], 0),
                i32x4::load(&arr[3], 0),
                i32x4::load(&arr[4], 0),
                i32x4::load(&arr[5], 0),
                i32x4::load(&arr[6], 0),
                i32x4::load(&arr[7], 0),
                i32x4::load(&arr[8], 0),
                i32x4::load(&arr[9], 0),
                i32x4::load(&arr[10], 0),
                i32x4::load(&arr[11], 0),
                i32x4::load(&arr[12], 0),
                i32x4::load(&arr[13], 0),
                i32x4::load(&arr[14], 0),
                i32x4::load(&arr[15], 0),
            ];

            for (x, &a) in columns.iter().enumerate() {
                let aps = unsafe { _mm_cvtepi32_ps(a) };
                for (y, &b) in columns.iter().enumerate() {
                    if x == y { continue; }

                    let bps = unsafe { _mm_cvtepi32_ps(b) };
                    
                    // convert and round to simulate integer division
                    let cps = unsafe { _mm_div_ps(aps, bps) };
                    // stdsimd _mm_round_ps is bugged..
                    let cps = unsafe { _mm_cvttps_epi32(cps) };
                    let cps = unsafe { _mm_cvtepi32_ps(cps) };
                    let dps = unsafe { _mm_mul_ps(cps, bps) };

                    let e: u8x16 = unsafe { ::std::mem::transmute(_mm_cmpeq_ps(aps, dps)) };
                    let c = unsafe { _mm_cvttps_epi32(cps) };
                    let e = unsafe { _mm_and_si128(c.into(), ::std::mem::transmute(e)) };

                    checksums = unsafe { _mm_add_epi32(checksums, e.into()) };
                }
            }

            row = 0;
        }
    }

    checksums = unsafe { _mm_hadd_epi32(checksums, checksums) };
    checksums = unsafe { _mm_hadd_epi32(checksums, checksums) };

    println!("{:?}", checksums.extract(0));
}
