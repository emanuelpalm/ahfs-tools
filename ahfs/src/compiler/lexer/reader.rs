use super::{Lexeme, LexemeKind, Region, Text};

/// A utility for creating [`Lexeme`s](struct.Lexeme.html) from source texts.
///
/// # Operation
///
/// Extracts _lexemes_ from a _source_ text. When created, it contains a
/// _candidate lexeme_ with length 0 at the beginning of its text. The candidate
/// lexeme can be expanded to include more bytes, and later collected or
/// discarded when it includes some set of significant characters. If collected,
/// the candidate is returned. If either collected or discarded, a new
/// zero-length candidate is created at the end position of the old one.
#[derive(Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    text: &'a Text<'a>,
    start: usize,
    end: usize,
}

impl<'a> Reader<'a> {
    /// Creates new `Reader` from given source code `text` and `source_index`.
    #[inline]
    pub fn new(text: &'a Text<'a>) -> Self {
        Reader { bytes: text.body().as_bytes(), text, start: 0, end: 0 }
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

    /// Collects current candidate lexeme.
    ///
    /// # Panics
    ///
    /// If the current `start` and `end` offsets do not align with the UTF-8
    /// code boundaries of the source string, the method panics.
    #[inline]
    pub fn collect(&mut self, kind: LexemeKind) -> Lexeme<'a> {
        let lexeme = Lexeme::new(kind, unsafe {
            Region::new(self.text, self.start..self.end)
        });
        self.discard();
        lexeme
    }

    /// Discards current candidate lexeme.
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
        let mut reader = Reader::new(&text);

        // Skip As.
        assert_eq!(Some(b'a'), reader.next());
        assert_eq!(Some(b'a'), reader.next());
        reader.discard();

        // Take Bs.
        assert_eq!(Some(b'b'), reader.next());
        assert_eq!(Some(b'b'), reader.next());
        let lexeme = reader.collect(LexemeKind::Word);
        assert_eq!("bb", text.get(lexeme).unwrap());
    }
}
