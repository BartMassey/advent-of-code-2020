/// Given a byte slice starting with a valid UTF-8 character,
/// return the character and its byte length.
///
/// Shamelessly cribbed from the rust std library source.
pub fn utf8_char(bytes: &[u8]) -> Option<(usize, char)> {
    // Mask for low bits.
    const CONT_MASK: u8 = 0b0011_1111;

    let nbytes = bytes.len();
    if nbytes == 0 {
        return None;
    }

    // Decode ASCII.
    let x = bytes[0];
    if x < 128 {
        return Some((1, x as char));
    }

    fn utf8_first_byte(byte: u8, width: u32) -> u32 {
        (byte & (0x7F >> width)) as u32
    }

    fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
        (ch << 6) | (byte & CONT_MASK) as u32
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]

    // Number of bytes used in the encoding.
    let mut nused = 1;
    let init = utf8_first_byte(x, 2);
    if nbytes < nused + 1 {
        return None;
    }
    let y = bytes[nused];
    nused += 1;

    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        if nbytes < nused + 1 {
            return None;
        }
        let z = bytes[nused];
        nused += 1;

        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            if nbytes < nused + 1 {
                return None;
            }
            let w = bytes[nused];
            nused += 1;

            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    std::char::from_u32(ch).map(|c| (nused, c))
}

pub struct CharSource {
    s: String,
    i: usize,
}

impl CharSource {
    fn new(s: String) -> Self {
        CharSource { s, i: 0 }
    }
}

impl Iterator for CharSource {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let suffix = &self.s[self.i..].as_bytes();
        utf8_char(suffix).map(|(i, ch)| {
            self.i += i;
            ch
        })
    }
}

pub trait IntoChars {
    fn into_chars(self) -> CharSource;
}

impl IntoChars for String {
    fn into_chars(self) -> CharSource {
        CharSource::new(self)
    }
}

impl IntoChars for &'_ str {
    fn into_chars(self) -> CharSource {
        CharSource::new(self.to_owned())
    }
}

#[test]
fn test_into_chars() {
    let s = "hello 🌎!";
    let t: String = s.into_chars().collect();
    assert_eq!(s, t);
}
