use std::str;
use super::Lexeme;

/// A utility for creating [`Lexeme`s](struct.Lexeme.html) from source strings.
///
/// # Operation
///
/// Extracts _lexemes_ from a _source_ string. When created, it contains a
/// _candidate lexeme_ with length 0 at the beginning of its source. The
/// candidate lexeme can be expanded to include more bytes, and later collected
/// or discarded when it includes some set of significant characters. If
/// collected, the candidate is returned. If either collected or discarded,
/// a new zero-length candidate is created at the end position of the old one.
#[derive(Debug)]
pub struct Source<'a> {
    source: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Source<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Source { source: source.as_bytes(), start: 0, end: 0 }
    }

    /// Reads byte right after current candidate `Lexeme`.
    #[inline]
    pub fn read(&mut self) -> Option<u8> {
        self.source.get(self.end).map(|byte| *byte)
    }

    /// Expands candidate `Lexeme`, making it include one more byte.
    #[inline]
    pub fn next(&mut self) {
        self.end += 1;
    }

    /// Collects current candidate lexeme.
    #[inline]
    pub fn collect<K>(&mut self, kind: K) -> Lexeme<K> {
        let lexeme = Lexeme::new(kind, self.start, self.end);
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
    use super::*;

    #[test]
    fn collect() {
        let source_str = "aabbcc";
        let mut source = Source::new(source_str);

        // Skip As.
        assert_eq!(Some(b'a'), source.read());
        source.next();
        assert_eq!(Some(b'a'), source.read());
        source.next();
        source.discard();

        // Take Bs.
        assert_eq!(Some(b'b'), source.read());
        source.next();
        assert_eq!(Some(b'b'), source.read());
        source.next();
        let lexeme = source.collect(());
        assert_eq!("bb", lexeme.extract(source_str));
    }
}
