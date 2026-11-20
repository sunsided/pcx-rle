use pcx_rle::{decode, encode};
use proptest::prelude::*;

proptest! {
    /// Whatever goes in comes back out.
    #[test]
    fn round_trip(src in prop::collection::vec(any::<u8>(), 0..512)) {
        prop_assert_eq!(decode(&encode(&src)).unwrap(), src);
    }

    /// Never more than twice the size: one escaped byte is the worst case.
    #[test]
    fn at_most_double(src in prop::collection::vec(any::<u8>(), 0..512)) {
        prop_assert!(encode(&src).len() <= 2 * src.len());
    }

    /// Decoding garbage may fail, but never panics.
    #[test]
    fn decode_never_panics(src in prop::collection::vec(any::<u8>(), 0..512)) {
        let _ = decode(&src);
    }
}
