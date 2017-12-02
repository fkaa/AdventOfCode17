extern crate stdsimd;

fn main() {
    use std::io::BufRead;
    use std::str::FromStr;

    use stdsimd::vendor::*;
    use stdsimd::simd::*;

    let stdin = ::std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    let mut arr = [[0i16; 8]; 16];
    let mut row = 0;
    let mut checksums = i16x8::splat(0);

    while let Ok(n) = stdin.read_line(&mut buf) {
        if n == 0 { break; }
        buf.split_whitespace().map(|s| i16::from_str(&s).unwrap()).enumerate().for_each(|(idx, n)| arr[idx][row] = n);
        buf.clear();
        row += 1;

        if row == 8 {
            let columns = [
                i16x8::load(&arr[0], 0),
                i16x8::load(&arr[1], 0),
                i16x8::load(&arr[2], 0),
                i16x8::load(&arr[3], 0),
                i16x8::load(&arr[4], 0),
                i16x8::load(&arr[5], 0),
                i16x8::load(&arr[6], 0),
                i16x8::load(&arr[7], 0),
                i16x8::load(&arr[8], 0),
                i16x8::load(&arr[9], 0),
                i16x8::load(&arr[10], 0),
                i16x8::load(&arr[11], 0),
                i16x8::load(&arr[12], 0),
                i16x8::load(&arr[13], 0),
                i16x8::load(&arr[14], 0),
                i16x8::load(&arr[15], 0),
            ];


            let mut min = columns[0];
            for &col in columns.iter() {
                min = unsafe { _mm_min_epi16(min, col) };
            }

            let mut max = columns[0];
            for &col in columns.iter() {
                max = unsafe { _mm_max_epi16(max, col) };
            }

            let checksum = unsafe { _mm_sub_epi16(max, min) };

            checksums = unsafe { _mm_add_epi16(checksums, checksum) };

            row = 0;
        }
    }

    checksums = unsafe { _mm_hadd_epi16(checksums, checksums) };
    checksums = unsafe { _mm_hadd_epi16(checksums, checksums) };

    println!("{}", checksums.extract(0));
}
