extern crate stdsimd;

fn main() {
    use std::io::BufRead;
    use std::str::FromStr;

    use stdsimd::vendor::*;
    use stdsimd::simd::*;

    let stdin = ::std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    let mut arr = [[0i16; 16]; 16];
    let mut row = 0;

    while let Ok(n) = stdin.read_line(&mut buf) {
        if n == 0 { break; }
        buf.split_whitespace().map(|s| i16::from_str(&s).unwrap()).enumerate().for_each(|(idx, n)| arr[idx][row] = n);
        buf.clear();
        row += 1;

        if row == 16 {
            let a = i16x16::load(&arr[0], 0);
            let b = i16x16::load(&arr[1], 0);
            let c = i16x16::load(&arr[2], 0);
            let d = i16x16::load(&arr[3], 0);
            let e = i16x16::load(&arr[4], 0);
            let f = i16x16::load(&arr[5], 0);
            let g = i16x16::load(&arr[6], 0);
            let h = i16x16::load(&arr[7], 0);
            let i = i16x16::load(&arr[8], 0);
            let j = i16x16::load(&arr[9], 0);
            let k = i16x16::load(&arr[10], 0);
            let l = i16x16::load(&arr[11], 0);
            let m = i16x16::load(&arr[12], 0);
            let n = i16x16::load(&arr[13], 0);
            let o = i16x16::load(&arr[14], 0);
            let p = i16x16::load(&arr[15], 0);

            let mut min = a;
            min = unsafe { _mm256_min_epi16(min, b) };
            min = unsafe { _mm256_min_epi16(min, c) };
            min = unsafe { _mm256_min_epi16(min, d) };
            min = unsafe { _mm256_min_epi16(min, e) };
            min = unsafe { _mm256_min_epi16(min, f) };
            min = unsafe { _mm256_min_epi16(min, g) };
            min = unsafe { _mm256_min_epi16(min, h) };
            min = unsafe { _mm256_min_epi16(min, i) };
            min = unsafe { _mm256_min_epi16(min, j) };
            min = unsafe { _mm256_min_epi16(min, k) };
            min = unsafe { _mm256_min_epi16(min, l) };
            min = unsafe { _mm256_min_epi16(min, m) };
            min = unsafe { _mm256_min_epi16(min, n) };
            min = unsafe { _mm256_min_epi16(min, o) };
            min = unsafe { _mm256_min_epi16(min, p) };

            let mut max = a;
            max = unsafe { _mm256_max_epi16(max, b) };
            max = unsafe { _mm256_max_epi16(max, c) };
            max = unsafe { _mm256_max_epi16(max, d) };
            max = unsafe { _mm256_max_epi16(max, e) };
            max = unsafe { _mm256_max_epi16(max, f) };
            max = unsafe { _mm256_max_epi16(max, g) };
            max = unsafe { _mm256_max_epi16(max, h) };
            max = unsafe { _mm256_max_epi16(max, i) };
            max = unsafe { _mm256_max_epi16(max, j) };
            max = unsafe { _mm256_max_epi16(max, k) };
            max = unsafe { _mm256_max_epi16(max, l) };
            max = unsafe { _mm256_max_epi16(max, m) };
            max = unsafe { _mm256_max_epi16(max, n) };
            max = unsafe { _mm256_max_epi16(max, o) };
            max = unsafe { _mm256_max_epi16(max, p) };

            let checksum = unsafe { _mm256_sub_epi16(max, min) };

            let checksum = unsafe { _mm256_hadd_epi16(checksum, checksum) };
            let checksum = unsafe { _mm256_hadd_epi16(checksum, checksum) };

            println!("{}", checksum.extract(0));

            row = 0;
        }
    }
}
