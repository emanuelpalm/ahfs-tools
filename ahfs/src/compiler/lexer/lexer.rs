pub use super::Lexeme;

use std::char;
use std::str;

/// A lexical analyzer.
///
/// # Operation
///
/// A lexer extracts _lexemes_ from a _source_ string. When created, the lexer
/// contains a _candidate lexeme_ with length 0 at the beginning of its source.
/// The candidate lexeme can be expanded to include more characters, and later
/// collected or discarded when it includes some set of signifcant characters.
/// If collected, the candidate is saved to a vector. If either collected or
/// discarded, a new 0 length candidate is created right after the old one.
///
/// ## Example
///
/// ```rust
/// use ahfs::compiler::Lexer;
///
/// let mut lexer = Lexer::new("aabbaa");
///
/// // Skip As.
/// assert_eq!(Some('a'), lexer.next());
/// assert_eq!(Some('a'), lexer.next());
/// lexer.discard();
///
/// // Take Bs.
/// assert_eq!(Some('b'), lexer.next());
/// assert_eq!(Some('b'), lexer.next());
/// lexer.collect();
///
/// // Finalize analysis.
/// let lexemes = lexer.into_lexemes();
/// assert_eq!(1, lexemes.len());
/// assert_eq!("bb", lexemes[0].as_str());
/// ```
pub struct Lexer<'a> {
    candidate: Candidate<'a>,
    collected: Vec<Lexeme<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates new lexer for analyzing given source string.
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Lexer {
            candidate: Candidate::new(source),
            collected: Vec::new(),
        }
    }

    /// Expands candidate to include one more character, which is also returned.
    #[inline]
    pub fn next(&mut self) -> Option<char> {
        self.candidate.next()
    }

    /// Collects current candidate lexeme.
    pub fn collect(&mut self) {
        self.collected.push(self.candidate.collect());
    }

    /// Discards current candidate lexeme.
    #[inline]
    pub fn discard(&mut self) {
        self.candidate.discard();
    }

    /// Consumes lexer and returns all lexemes it collected.
    pub fn into_lexemes(self) -> Vec<Lexeme<'a>> {
        self.collected
    }
}

struct Candidate<'a> {
    source: &'a [u8],
    offset: usize,
    end: usize,
}

impl<'a> Candidate<'a> {
    #[inline]
    fn new(source: &'a str) -> Self {
        Candidate { source: source.as_bytes(), offset: 0, end: 0 }
    }

    fn next(&mut self) -> Option<char> {
        let a = self.next_byte()?;

        // Is 1 byte character?
        if a < 128 {
            return Some(unsafe { char::from_u32_unchecked(a as u32) });
        }
        // 2+ byte character!

        let init = (a & 0b0001_1111) as u32;
        let b = self.next_byte_or_0();
        let mut code = utf8_accumulate(init, b);

        // Is 3+ byte character?
        if a >= 0xE0 {
            let c = self.next_byte_or_0();
            let bc = utf8_accumulate((b & UTF8_CONT_MASK) as u32, c);
            code = init << 12 | bc;

            // Is 4 byte character?
            if a >= 0xF0 {
                let d = self.next_byte_or_0();
                code = (init & 0b111) << 18 | utf8_accumulate(bc, d);
            }
        }
        return Some(unsafe { char::from_u32_unchecked(code) });

        const UTF8_CONT_MASK: u8 = 0b0011_1111;

        #[inline]
        fn utf8_accumulate(code: u32, byte: u8) -> u32 {
            (code << 6) | (byte & UTF8_CONT_MASK) as u32
        }
    }

    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        if self.end >= self.source.len() {
            return None;
        }
        let byte = *unsafe { self.source.get_unchecked(self.end) };
        self.end += 1;
        Some(byte)
    }

    #[inline]
    fn next_byte_or_0(&mut self) -> u8 {
        if self.end >= self.source.len() {
            return 0;
        }
        let byte = *unsafe { self.source.get_unchecked(self.end) };
        self.end += 1;
        byte
    }

    #[inline]
    fn collect(&mut self) -> Lexeme<'a> {
        let source = unsafe { str::from_utf8_unchecked(self.source) };
        let len = self.end - self.offset;
        let lexeme = Lexeme::new(source, self.offset, len);
        self.discard();
        lexeme
    }

    #[inline]
    fn discard(&mut self) {
        self.offset = self.end;
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
