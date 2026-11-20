//! PCX run-length coding, as ZSoft's PC Paintbrush wrote it: a byte with both
//! top bits set is a count in its low six bits, and the byte after it is the
//! value to repeat. Anything else is a literal, so a literal of 0xC0 or more
//! has to be written as a run of one.

/// The longest run a count byte can say: six bits.
pub const MAX_RUN: usize = 63;

/// Encodes `src` into PCX runs.
pub fn encode(src: &[u8]) -> Vec<u8> {
    let mut out = vec![0; 2 * src.len()];
    let len = encode_into(src, &mut out);
    out.truncate(len);
    out
}

/// Encodes `src` into `out`, which needs room for twice its length, and returns
/// how many bytes it wrote.
pub fn encode_into(src: &[u8], out: &mut [u8]) -> usize {
    let mut o = 0;
    let mut i = 0;
    while i < src.len() {
        let v = src[i];
        let mut n = 1;
        while i + n < src.len() && src[i + n] == v && n < limit() {
            n += 1;
        }
        if n > 1 || needs_escape(v) {
            out[o] = 0xC0 | n as u8;
            out[o + 1] = v;
            o += 2;
        } else {
            out[o] = v;
            o += 1;
        }
        i += n;
    }
    o
}

/// The longest run the encoder writes.
#[cfg(not(feature = "bug-cap"))]
const fn limit() -> usize {
    MAX_RUN
}

/// Off by one: a run of 64 comes out as `0xC0 | 64`, which is a count of zero.
#[cfg(feature = "bug-cap")]
const fn limit() -> usize {
    MAX_RUN + 1
}

/// Whether a single byte has to go out as a run of one.
#[cfg(not(feature = "bug-escape"))]
const fn needs_escape(v: u8) -> bool {
    v >= 0xC0
}

/// The classic: forget that a literal can look like a count.
#[cfg(feature = "bug-escape")]
const fn needs_escape(_v: u8) -> bool {
    false
}

/// What can go wrong reading runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// A count byte at this offset is the last byte: its value is missing.
    Truncated { at: usize },
    /// The output does not fit the buffer it is decoded into.
    TooLong,
}

/// Decodes PCX runs into `out`, and returns how many bytes it wrote.
pub fn decode_into(src: &[u8], out: &mut [u8]) -> Result<usize, DecodeError> {
    let mut o = 0;
    let mut i = 0;
    while i < src.len() {
        let b = src[i];
        if b & 0xC0 == 0xC0 {
            let v = *src.get(i + 1).ok_or(DecodeError::Truncated { at: i })?;
            let n = usize::from(b & 0x3F);
            out.get_mut(o..o + n).ok_or(DecodeError::TooLong)?.fill(v);
            o += n;
            i += 2;
        } else {
            *out.get_mut(o).ok_or(DecodeError::TooLong)? = b;
            o += 1;
            i += 1;
        }
    }
    Ok(o)
}

/// How many bytes `src` decodes to.
pub fn decoded_len(src: &[u8]) -> Result<usize, DecodeError> {
    let mut len = 0;
    let mut i = 0;
    while i < src.len() {
        let b = src[i];
        if b & 0xC0 == 0xC0 {
            if i + 1 >= src.len() {
                return Err(DecodeError::Truncated { at: i });
            }
            len += usize::from(b & 0x3F);
            i += 2;
        } else {
            len += 1;
            i += 1;
        }
    }
    Ok(len)
}

/// Decodes PCX runs.
pub fn decode(src: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let mut out = vec![0; decoded_len(src)?];
    decode_into(src, &mut out)?;
    Ok(out)
}

/// The same, with the bounds checks taken out, since `decoded_len` has
/// already seen every byte. Faster on long runs, and the reason Miri exists.
pub fn decode_fast(src: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let len = fast_len(src)?;
    let mut out = Vec::<u8>::with_capacity(len);
    let (src_ptr, dst) = (src.as_ptr(), out.as_mut_ptr());
    let (mut i, mut o) = (0, 0);
    // SAFETY: every count byte has its value after it, and the runs add up to
    // exactly `len`, the capacity: `fast_len` checked both.
    unsafe {
        while i < src.len() {
            let b = src_ptr.add(i).read();
            if b & 0xC0 == 0xC0 {
                let n = usize::from(b & 0x3F);
                dst.add(o).write_bytes(src_ptr.add(i + 1).read(), n);
                o += n;
                i += 2;
            } else {
                dst.add(o).write(b);
                o += 1;
                i += 1;
            }
        }
        out.set_len(len);
    }
    Ok(out)
}

/// The first pass of `decode_fast`.
#[cfg(not(feature = "bug-unchecked"))]
fn fast_len(src: &[u8]) -> Result<usize, DecodeError> {
    decoded_len(src)
}

/// A first pass that counts runs but never checks the last one has a value.
#[cfg(feature = "bug-unchecked")]
fn fast_len(src: &[u8]) -> Result<usize, DecodeError> {
    let (mut len, mut i) = (0, 0);
    while i < src.len() {
        if src[i] & 0xC0 == 0xC0 {
            len += usize::from(src[i] & 0x3F);
            i += 2;
        } else {
            len += 1;
            i += 1;
        }
    }
    Ok(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_and_literals() {
        assert_eq!(encode(b"AAAAB"), [0xC4, b'A', b'B']);
        assert_eq!(decode(&[0xC4, b'A', b'B']).unwrap(), b"AAAAB");
    }

    #[test]
    fn empty() {
        assert!(encode(&[]).is_empty());
        assert!(decode(&[]).unwrap().is_empty());
    }

    #[test]
    fn truncated_run() {
        assert_eq!(decode(&[b'A', 0xC3]), Err(DecodeError::Truncated { at: 1 }));
    }
}

#[cfg(kani)]
mod proofs;
