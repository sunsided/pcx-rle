use pcx_rle::{decode, encode};
use proptest::prelude::*;

fn bytes() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(prop_oneof![Just(0u8), Just(0xC0), any::<u8>()], 0..256)
}

proptest! {
    /// Packets stand alone: two encodings glued together decode to the two inputs glued together.
    #[test]
    fn concatenation(a in bytes(), b in bytes()) {
        let mut glued = encode(&a);
        glued.extend(encode(&b));
        prop_assert_eq!(decode(&glued).unwrap(), [a, b].concat());
    }

    /// Encoding the whole never costs more than encoding the halves: runs can only merge.
    #[test]
    fn whole_is_no_larger(a in bytes(), b in bytes()) {
        let whole = encode(&[a.clone(), b.clone()].concat()).len();
        prop_assert!(whole <= encode(&a).len() + encode(&b).len());
    }

    /// Renaming bytes doesn't change the shape: XOR with a key below 0x40 keeps every
    /// byte on its side of 0xC0, and so keeps the encoded length.
    #[test]
    fn renaming_keeps_length(src in bytes(), key in 0u8..0x40) {
        let renamed: Vec<u8> = src.iter().map(|b| b ^ key).collect();
        prop_assert_eq!(encode(&renamed).len(), encode(&src).len());
    }
}
