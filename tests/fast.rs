use pcx_rle::{DecodeError, decode_fast};

#[test]
fn fast_truncated_run() {
    assert_eq!(decode_fast(&[b'A', 0xC3]), Err(DecodeError::Truncated { at: 1 }));
}
