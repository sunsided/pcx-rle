use pcx_rle::{decode, encode};

#[test]
fn round_trips() {
    let cases: &[&[u8]] = &[
        b"",
        b"A",
        b"AAAAB",
        b"ABABAB",
        b"Hello, world!",
        &[0x00; 63],
        &[0xFF, 0xFF, 0xFF],
    ];
    for &src in cases {
        assert_eq!(decode(&encode(src)).unwrap(), src, "{src:02x?}");
    }
}
