use std::char;
use std::str;
use std::u32;
use super::Lexeme;

/// A utility for creating [`Lexeme`s](struct.Lexeme.html) from source strings.
///
/// # Operation
///
/// Extracts _lexemes_ from a _source_ string. When created, it contains a
/// _candidate lexeme_ with length 0 at the beginning of its source. The
/// candidate lexeme can be expanded to include more characters, and later
/// collected or discarded when it includes some set of significant characters.
/// If collected, the candidate is returned. If either collected or discarded,
/// a new zero-length candidate is created at the end position of the old one.
///
/// ## Example
///
/// ```rust
/// use ahfs::compiler::lexer::Lexer;
///
/// let source = "aabbaa";
/// let mut lexer = Lexer::new(source);
///
/// // Skip As.
/// assert_eq!(Some('a'), lexer.next());
/// assert_eq!(Some('a'), lexer.next());
/// lexer.discard();
///
/// // Take Bs.
/// assert_eq!(Some('b'), lexer.next());
/// assert_eq!(Some('b'), lexer.next());
/// let lexeme = lexer.collect(());
/// assert_eq!("bb", lexeme.extract(source));
/// ```
#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    stop: usize,
}

impl<'a> Lexer<'a> {
    /// Creates new lexer for analyzing given source string.
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Lexer { source: source.as_bytes(), start: 0, stop: 0 }
    }

    /// Expands candidate to include one more character, which is also returned.
    #[inline]
    pub fn next(&mut self) -> Option<char> {
        let code = loop {
            let a = self.next_byte()?;

            // Is 1 byte character?
            if a < 128 {
                break a as u32;
            }

            let init = (a & 0b0001_1111) as u32;
            let b = self.next_byte_or_0();

            // Is 2 byte character?
            if a < 0xe0 {
                break utf8_push(init, b);
            }

            let c = self.next_byte_or_0();
            let b_c = utf8_push((b & UTF8_CONT_MASK) as u32, c);

            // Is 3 byte character?
            if a < 0xf0 {
                break init << 12 | b_c;
            }

            // Is 4 byte character!
            let d = self.next_byte_or_0();
            break (init & 0b111) << 18 | utf8_push(b_c, d);
        };
        return Some(unsafe { char::from_u32_unchecked(code) });

        const UTF8_CONT_MASK: u8 = 0b0011_1111;

        #[inline]
        fn utf8_push(code: u32, byte: u8) -> u32 {
            (code << 6) | (byte & UTF8_CONT_MASK) as u32
        }
    }

    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        if self.stop >= self.source.len() {
            return None;
        }
        let byte = *unsafe { self.source.get_unchecked(self.stop) };
        self.stop += 1;
        Some(byte)
    }

    #[inline]
    fn next_byte_or_0(&mut self) -> u8 {
        if self.stop >= self.source.len() {
            return 0;
        }
        let byte = *unsafe { self.source.get_unchecked(self.stop) };
        self.stop += 1;
        byte
    }

    /// Shrinks candidate, making it include one less character.
    #[inline]
    pub fn undo(&mut self) {
        if self.start == self.stop {
            return;
        }
        loop {
            self.stop -= 1;
            let byte = *unsafe { self.source.get_unchecked(self.stop) };
            if !utf8_is_cont_byte(byte) {
                return;
            }
        }

        #[inline]
        fn utf8_is_cont_byte(byte: u8) -> bool {
            (byte & 0b1100_0000) == 0b1000_0000
        }
    }

    /// Collects current candidate lexeme.
    #[inline]
    pub fn collect<K>(&mut self, kind: K) -> Lexeme<K> {
        let lexeme = Lexeme::new(kind, self.start, self.stop);
        self.discard();
        lexeme
    }

    /// Discards current candidate lexeme.
    #[inline]
    pub fn discard(&mut self) {
        self.start = self.stop;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next() {
        let mut lexer = Lexer::new("$¢€𐍈");
        assert_eq!(Some('$'), lexer.next());
        assert_eq!(Some('¢'), lexer.next());
        assert_eq!(Some('€'), lexer.next());
        assert_eq!(Some('𐍈'), lexer.next());
        assert_eq!(None, lexer.next());
        assert_eq!(None, lexer.next());
    }
}
