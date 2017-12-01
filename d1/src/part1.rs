extern crate stdsimd;

fn main() {
    use stdsimd::vendor::*;
    use stdsimd::simd::*;

    let data = include_bytes!("../input");
    let mut input = data.to_vec();
    input.extend(&data[0..1]);

    let len = input.len() / 16;

    let mut sum = i16x8::splat(0);

    // we iterate over N elements at a time:
    for i in 0..len {
        let index = i * 16;

        // get both `a` and `b` into "numberspace"
        let a = __m128i::from(u8x16::load(&input, index));
        let a = unsafe { _mm_sub_epi8(a, i8x16::splat('0' as i8)) };

        let b = __m128i::from(u8x16::load(&input, index + 1));
        let b = unsafe { _mm_sub_epi8(b, i8x16::splat('0' as i8)) };

        // compare `a` and `b`, result is either 1 or 0 (bits)
        let c = unsafe { _mm_cmpeq_epi8(a, b) };
        // do bitwise AND on `a` with our "mask" `c` to get all the sequnces in
        // our vector
        let c = unsafe { _mm_and_si128(a, c) };

        // move from one i8x16 to two i16x8
        let lo = unsafe { _mm_unpacklo_epi8(c, i8x16::splat(0)) };
        let hi = unsafe { _mm_unpackhi_epi8(c, i8x16::splat(0)) };

        sum = unsafe { _mm_add_epi16(sum, lo.into()) };
        sum = unsafe { _mm_add_epi16(sum, hi.into()) };
    }

    sum = unsafe { _mm_hadd_epi16(sum, sum) };
    sum = unsafe { _mm_hadd_epi16(sum, sum) };

    let sum = sum.extract(0);

    println!("{}", sum);
}
