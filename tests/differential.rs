use pcx_rle::{DecodeError, decode, decode_fast};
use proptest::prelude::*;

/// The decoder from "Sixty-four thousand bytes", as printed there: it is told how
/// much to expect, and it never fails.
fn unpack(src: &[u8], out_len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(out_len);
    let mut it = src.iter().copied();
    while out.len() < out_len {
        let Some(b) = it.next() else { break };
        if b & 0xC0 == 0xC0 {
            let v = it.next().unwrap_or(0);
            out.extend(std::iter::repeat_n(v, (b & 0x3F) as usize));
        } else {
            out.push(b);
        }
    }
    out.truncate(out_len);
    out
}

proptest! {
    /// The fast decoder and the plain one agree on everything, errors included.
    #[test]
    fn fast_agrees(src in prop::collection::vec(any::<u8>(), 0..512)) {
        prop_assert_eq!(decode_fast(&src), decode(&src));
    }

    /// Where the plain decoder succeeds, the article's decoder says the same.
    #[test]
    fn article_agrees(src in prop::collection::vec(any::<u8>(), 0..512)) {
        if let Ok(ours) = decode(&src) {
            prop_assert_eq!(unpack(&src, ours.len()), ours);
        }
    }

    /// Where it fails, the article's decoder should at least not make bytes up. It does:
    /// `[0xC1]` comes out as `[0]`. Run with `--ignored` to see it.
    #[test]
    #[ignore = "documents a disagreement"]
    fn article_on_truncated(src in prop::collection::vec(any::<u8>(), 0..512)) {
        if let Err(DecodeError::Truncated { at }) = decode(&src) {
            let before = decode(&src[..at]).unwrap();
            prop_assert_eq!(unpack(&src, 63 * src.len()), before);
        }
    }
}
