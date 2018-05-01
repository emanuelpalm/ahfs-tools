use super::{Name, Region, Text, Token};

/// A utility for creating [`Token`s](struct.Token.html) from source texts.
///
/// # Operation
///
/// Extracts _tokens_ from a _source_ text. When created, it contains a
/// _candidate token_ with length 0 at the beginning of its text. The candidate
/// token can be expanded to include more bytes, and later collected or
/// discarded when it includes some set of significant characters. If collected,
/// the candidate is returned. If either collected or discarded, a new
/// zero-length candidate is created at the end position of the old one.
#[derive(Debug)]
pub struct Scanner<'a> {
    bytes: &'a [u8],
    text: &'a Text,
    start: usize,
    end: usize,
}

impl<'a> Scanner<'a> {
    /// Creates new `Scanner` from given source code `text`.
    #[inline]
    pub fn new(text: &'a Text) -> Self {
        Scanner { bytes: text.body().as_bytes(), text, start: 0, end: 0 }
    }

    /// Returns byte right after candidate, and then expands the candidate.
    #[inline]
    pub fn next(&mut self) -> Option<u8> {
        self.bytes.get(self.end).map(|byte| {
            self.end += 1;
            *byte
        })
    }

    /// Returns byte right after candidate.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.end).map(|byte| *byte)
    }

    /// Expands candidate, making it include one more byte.
    pub fn skip(&mut self) {
        self.end += 1;
    }

    /// Collects current candidate token.
    ///
    /// # Panics
    ///
    /// If the current `start` and `end` offsets do not align with the UTF-8
    /// code boundaries of the source string, the method panics.
    #[inline]
    pub fn collect(&mut self, kind: Name) -> Token<'a> {
        let token = Token::new(kind, unsafe {
            Region::new(self.text, self.start..self.end)
        });
        self.discard();
        token
    }

    /// Discards current candidate token.
    #[inline]
    pub fn discard(&mut self) {
        self.start = self.end;
    }
}

#[cfg(test)]
mod tests {
    use super::super::Text;
    use super::*;

    #[test]
    fn collect() {
        let text = Text::new("", "aabbcc");
        let mut reader = Scanner::new(&text);

        // Skip As.
        assert_eq!(Some(b'a'), reader.next());
        assert_eq!(Some(b'a'), reader.next());
        reader.discard();

        // Take Bs.
        assert_eq!(Some(b'b'), reader.next());
        assert_eq!(Some(b'b'), reader.next());
        let token = reader.collect(Name::Word);
        assert_eq!("bb", text.get(token).unwrap());
    }
}
