//! Kani proof harnesses: `cargo kani --exact --harness proofs::<name>`.

use super::*;

/// Every input of exactly N bytes survives the round trip, on fixed-size buffers.
macro_rules! round_trip {
    ($name:ident, $n:literal, $unwind:literal) => {
        #[kani::proof]
        #[kani::unwind($unwind)]
        fn $name() {
            let src: [u8; $n] = kani::any();
            let mut packed = [0; 2 * $n];
            let len = encode_into(&src, &mut packed);
            let mut out = [0; $n];
            assert!(decode_into(&packed[..len], &mut out) == Ok($n));
            assert!(out == src);
        }
    };
}
round_trip!(round_trip_4, 4, 7);
round_trip!(round_trip_8, 8, 11);
round_trip!(round_trip_16, 16, 19);
round_trip!(round_trip_32, 32, 35);
round_trip!(round_trip_64, 64, 67);

/// The same property against the `Vec` API, for every input of up to N bytes.
macro_rules! round_trip_vec {
    ($name:ident, $n:literal, $unwind:literal) => {
        #[kani::proof]
        #[kani::unwind($unwind)]
        fn $name() {
            let buf: [u8; $n] = kani::any();
            let len: usize = kani::any_where(|&l| l <= $n);
            let src = &buf[..len];
            assert!(decode(&encode(src)).as_deref() == Ok(src));
        }
    };
}
round_trip_vec!(round_trip_vec_1, 1, 4);
round_trip_vec!(round_trip_vec_2, 2, 5);
round_trip_vec!(round_trip_vec_4, 4, 7);

/// Decoding any 4 bytes into a buffer that is too small never panics.
#[kani::proof]
#[kani::unwind(66)]
fn decode_any_4() {
    let src: [u8; 4] = kani::any();
    let mut out = [0; 8];
    let _ = decode_into(&src, &mut out);
}

/// The fast decoder agrees with the plain one on every input of up to 4 bytes.
#[kani::proof]
#[kani::unwind(66)]
fn fast_agrees_4() {
    let buf: [u8; 4] = kani::any();
    let len: usize = kani::any_where(|&l| l <= 4);
    let src = &buf[..len];
    assert!(decode_fast(src) == decode(src));
}
