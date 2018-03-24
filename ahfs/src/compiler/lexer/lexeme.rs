use std::cmp;
use std::fmt;
use std::ops::Range;

/// Identifies a significant region within a source string.
#[derive(Debug)]
pub struct Lexeme<'a> {
    source: &'a str,
    offset: usize,
    len: usize, // TODO: Store _end_ internally instead of len?
}

impl<'a> Lexeme<'a> {
    /// Creates new lexeme from given source string, offset and length.
    ///
    /// # Panics
    ///
    /// ## Region Out of Bounds
    ///
    /// If the given offset and length cover a region outside the bounds of the
    /// given source string, the function panics.
    ///
    /// ## UTF-8 Code Point Boundaries
    ///
    /// Both of the given offset and length must refer to the first bytes of
    /// UTF-8 code points within the string. Failing to comply to this
    /// requirement will not cause an immediate panic, but will likely result
    /// in one when the lexeme is used later on.
    #[inline]
    pub fn new(source: &'a str, offset: usize, len: usize) -> Self {
        assert!(offset + len <= source.len());
        Lexeme { source, offset, len }
    }

    /// Borrows lexeme as string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.source[self.offset..(self.offset + self.len)]
    }

    /// Lexeme source string.
    #[inline]
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Offset, in bytes, from beginning of source string to lexeme.
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Length, in bytes, of lexeme.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Determines on which line the first byte of the lexeme is located within
    /// its source string.
    #[inline]
    pub fn row(&self) -> usize {
        self.source[..self.offset]
            .bytes()
            .filter(|b| *b == b'\n')
            .count() + 1
    }

    /// Creates new lexeme with source string truncated to only include lines
    /// touched by this lexeme.
    #[inline]
    pub fn shrink(&self) -> Lexeme<'a> {
        let start = match self.source[..self.offset].rfind('\n') {
            Some(index) => index + 1,
            None => 0,
        };
        let stop = match self.source[self.offset..].find('\n') {
            Some(index) => index + self.offset,
            None => self.offset,
        };
        Self::new(&self.source[start..stop], self.offset - start, self.len)
    }

    /// Combines regions of this and given lexeme.
    ///
    /// Any characters inbetween this and the referred lexeme are included by
    /// the returned lexeme.
    ///
    /// # Panics and Foreign Lexemes
    ///
    /// The given lexeme is not prevented from being foreign, mening it refers
    /// to a region within a different source string. The returned lexeme
    /// inherists its source string from this lexeme, which causes a panic if
    /// it would be too short to encompass the combined regions of the two
    /// lexemes.
    pub fn span(&self, other: &Lexeme<'a>) -> Lexeme<'a> {
        let offset = cmp::min(self.offset, other.offset);
        let end = cmp::max(self.offset + self.len, other.offset + other.len);
        Self::new(self.source, offset, end - offset)
    }
}

impl<'a> fmt::Display for Lexeme<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lexeme = self.shrink();
        let row = self.row();
        let mut x = row;

        write!(f, "     |\n")?;
        for line in lexeme.source().lines() {
            write!(f, "{:>4} | {}\n", x, line)?;
            x += 1;
        }
        // if row + 1 == x {
        //    write!(f, "     | {:offset$}{:^>len$}\n", "", "",
        //           offset = 
        //}
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &'static str = "A is: System;\nA consumes: B;\nA produces: C;\n";

    #[test]
    fn as_str() {
        let lexeme = Lexeme::new(DOC, 16, 9);
        assert_eq!("consumes:", lexeme.as_str());
    }

    #[test]
    fn display() {
        {
            let lexeme = Lexeme::new(DOC, 0, 1); 
            assert_eq!(
                format!("{}", lexeme).as_str(),
                "     |\n   1 | A is: System;\n     | ^\n");
        }
        {
            let lexeme = Lexeme::new(DOC, 16, 9);
            assert_eq!(
                format!("{}", lexeme).as_str(),
                "     |\n   2 | A consumes: B;\n     |   ^^^^^^^^^\n");
        }
        {
            let lexeme = Lexeme::new(DOC, 29, 14);
            assert_eq!(
                format!("{}", lexeme).as_str(),
                "     |\n   3 | A produces: C;\n     | ^^^^^^^^^^^^^^\n");
        }
    }
}
