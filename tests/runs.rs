use pcx_rle::{decode, encode};
use proptest::prelude::*;

/// Inputs made of runs, the way pictures are: a value and how often it repeats.
fn runs() -> impl Strategy<Value = Vec<u8>> {
    // One run: any byte value, repeated 1 to 199 times.
    let run = (any::<u8>(), 1..200usize);

    // Up to eight runs, written out one after another.
    prop::collection::vec(run, 0..8).prop_map(|runs| {
        let mut bytes = Vec::new();
        for (value, count) in runs {
            for _ in 0..count {
                bytes.push(value);
            }
        }
        bytes
    })
}

proptest! {
    #[test]
    fn round_trip_runs(src in runs()) {
        prop_assert_eq!(decode(&encode(&src)).unwrap(), src);
    }
}
