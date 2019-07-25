use crate::{Span, Text, Token};
use std::char;
use std::fmt;

/// A utility for creating [`Token`s](struct.Token.html) from source texts.
///
/// # Operation
///
/// Extracts _tokens_ from a source [`text`][txt]. When created, it contains a
/// _candidate token_ with length 0 at the beginning of its text. The candidate
/// token can be expanded to include more characters, and later collected or
/// discarded when it includes some set of significant characters. If collected,
/// the candidate is returned. If either collected or discarded, a new
/// zero-length candidate is created at the end position of the old one.
///
/// [txt]: struct.Text.html
pub struct Scanner<'a> {
    source: &'a Text,
    bytes: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Scanner<'a> {
    /// Creates new `Scanner` from given source code [`text`][txt].
    ///
    /// [txt]: struct.Text.html
    #[inline]
    pub fn new(source: &'a Text) -> Self {
        Scanner {
            source,
            bytes: source.body.as_bytes(),
            start: 0,
            end: 0,
        }
    }

    /// Returns character right after candidate, and then expands candidate.
    pub fn next(&mut self) -> Option<char> {
        let x = self.next_byte()?;
        if x < 128 {
            return Some(unsafe { char::from_u32_unchecked(x as u32) });
        }
        let init = (x & 0x1F) as u32;
        let y = (self.next_byte_or_0() & 0b0011_1111) as u32;
        let mut ch = (init << 6) | y;
        if x >= 0xE0 {
            let z = (self.next_byte_or_0() & 0b0011_1111) as u32;
            let y_z = (y << 6) | z;
            ch = init << 12 | y_z;
            if x >= 0xF0 {
                let w = (self.next_byte_or_0() & 0b0011_1111) as u32;
                ch = (init & 7) << 18 | (y_z << 6) | w;
            }
        }
        return Some(unsafe { char::from_u32_unchecked(ch) });
    }

    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        let x = self.bytes.get(self.end);
        self.end += 1;
        x.map(|x| *x)
    }

    #[inline]
    fn next_byte_or_0(&mut self) -> u8 {
        self.next_byte().unwrap_or(0)
    }

    /// Shrinks candidate by one character, unless candidate has length zero.
    pub fn unwind(&mut self) {
        loop {
            if self.start == self.end {
                break;
            }
            self.end -= 1;
            let byte = unsafe { *self.bytes.get_unchecked(self.end) };
            if (byte & 0b1100_0000) != 0b1000_0000 {
                break;
            }
        }
    }

    /// Collects current candidate token with provided `kind`.
    #[inline]
    pub fn collect<Kind>(&mut self, kind: Kind) -> Token<'a, Kind>
        where Kind: Copy + Clone + fmt::Debug,
    {
        let token = Token {
            class: kind,
            span: Span {
                source: self.source,
                range: (self.start..self.end).into()
            }
        };
        self.discard();
        token
    }

    /// Discards current candidate token.
    #[inline]
    pub fn discard(&mut self) {
        self.start = self.end;
    }

    /// Returns current candidate string, without consuming or discarding it.
    #[inline]
    pub fn review(&self) -> &str {
        unsafe {
            self.source.body.get_unchecked(self.start..self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect() {
        let source = Text {
            name: "".into(),
            body: "aabbccc".into(),
        };
        let mut reader = Scanner::new(&source);

        // Skip As.
        assert_eq!(Some('a'), reader.next());
        assert_eq!(Some('a'), reader.next());
        reader.discard();

        // Take Bs.
        assert_eq!(Some('b'), reader.next());
        assert_eq!(Some('b'), reader.next());
        let token = reader.collect(0);
        assert_eq!("bb", token.span.as_str());
        assert_eq!(0, token.class);

        // Take Cs.
        assert_eq!(Some('c'), reader.next());
        assert_eq!(Some('c'), reader.next());
        reader.unwind();
        let candidate = reader.review();
        assert_eq!("c", candidate);
    }
}
