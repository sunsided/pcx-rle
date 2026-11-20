#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &[u8]| {
    let packed = pcx_rle::encode(src);
    assert_eq!(pcx_rle::decode(&packed).unwrap(), src);
    // Any bytes at all are fair game for the decoders, and they must agree.
    assert_eq!(pcx_rle::decode_fast(src), pcx_rle::decode(src));
});
