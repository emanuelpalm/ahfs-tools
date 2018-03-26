use std::fmt;

/// Identifies a significant region within a source string.
#[derive(Debug)]
pub struct Lexeme<'a> {
    source: &'a str,
    start: usize,
    stop: usize,
}

impl<'a> Lexeme<'a> {
    /// Creates new lexeme from given source string, start offset and stop
    /// offset.
    ///
    /// The start offset points to the first byte of a lexeme located within
    /// the source string, while the stop offset points to the first byte after
    /// the lexeme.
    ///
    /// # Panics
    ///
    /// ## Region Out of Bounds
    ///
    /// If the given start offset is larger than the stop offset, or if the
    /// offsets cover a region outside the bounds of the given source string,
    /// the function panics.
    ///
    /// ## Invalid UTF-8 Code Point Boundaries
    ///
    /// Both of the given offsets must refer to the first bytes of UTF-8 code
    /// points within the string. Failing to comply to this requirement will
    /// causes an immediate panic only if running in debug mode. In release
    /// mode, a panic is likely going to be the result if using the methods of
    /// the lexeme later on.
    #[inline]
    pub fn new(source: &'a str, start: usize, stop: usize) -> Self {
        assert!(start <= stop && stop <= source.len());
        debug_assert!(source.is_char_boundary(start)
            && source.is_char_boundary(stop));

        Lexeme { source, start, stop }
    }

    /// Borrows lexeme as string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.source[self.start..self.stop]
    }

    /// Lexeme source string.
    #[inline]
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Start offset, in bytes, from beginning of source string to lexeme.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Stop offset, in bytes, from beginning of source string to first byte
    /// after lexeme.
    #[inline]
    pub fn stop(&self) -> usize {
        self.stop
    }

    /// Creates iterator over the source string lines touched by this lexeme.
    #[inline]
    pub fn lines(&self) -> Lines<'a> {
        Lines::new(self)
    }

    /// Determines line number of the first byte of this lexeme.
    #[inline]
    pub fn line_number(&self) -> usize {
        self.source[..self.start]
            .bytes()
            .filter(|b| *b == b'\n')
            .count() + 1
    }

    /// Combines regions of this and given lexeme.
    ///
    /// Any characters between this and the referred lexeme are included by the
    /// returned lexeme.
    ///
    /// # Foreign Lexemes
    ///
    /// The given lexeme is not prevented from referring to a region within
    /// some other source string. In any case, the returned lexeme will refer
    /// to a valid region within the same source string as this lexeme.
    pub fn span<'b>(&self, other: &Lexeme<'b>) -> Lexeme<'a> {
        use std::cmp::Ord;

        let start = self.start.min(other.start);
        let stop = self.stop.max(other.stop).min(self.source.len());
        Self::new(self.source, start, stop)
    }
}

impl<'a> fmt::Display for Lexeme<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "      |\n")?;
        for line in self.lines() {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Lines<'a> {
    source: &'a str,
    line_number: usize,
    start: usize,
    stop: usize,
    offset: usize,
}

impl<'a> Lines<'a> {
    #[inline]
    fn new(lexeme: &Lexeme<'a>) -> Self {
        let source = lexeme.source();

        // Find first newline before lexeme.
        let start = match source[..lexeme.start()].rfind('\n') {
            Some(index) => index + 1,
            None => 0,
        };

        // Find first newline after lexeme.
        let stop = lexeme.stop() + match source[lexeme.stop()..].find('\n') {
            Some(mut index) => {
                // Exclude any carriage return.
                if index > 0 && source.as_bytes()[index - 1] == b'\r' {
                    index -= 1;
                }
                index
            }
            None => 0,
        };

        Lines {
            source: &source[start..stop],
            line_number: source[..start]
                .bytes()
                .filter(|b| *b == b'\n')
                .count() + 1,
            start: lexeme.start() - start,
            stop: lexeme.stop() - start,
            offset: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.source.len() {
            return None;
        }

        let mut skip = 1;

        // Find end of line.
        let (mut offset, at_end) = match self.source[self.offset..].find('\n') {
            Some(index) => (index, false),
            None => (self.source.len(), true),
        };

        // Adjust line end if its last character is a carriage return.
        if offset > 0 && self.source.as_bytes()[offset - 1] == b'\r' {
            offset -= 1;
            skip += 1;
        }

        // Truncate source to cover only line.
        let source = &self.source[self.offset..offset];

        let line_number = self.line_number;

        // Determine start of lexeme within line.
        let start = if self.offset > 0 { 0 } else { self.start };

        // Determine stop of lexeme within line.
        let stop = if at_end { self.stop - self.offset } else { offset };

        // Forward internal offset and line_number.
        self.offset = offset + skip;
        self.line_number += 1;

        Some(Line { source, line_number, start, stop })
    }
}

#[derive(Debug)]
pub struct Line<'a> {
    source: &'a str,
    line_number: usize,
    start: usize,
    stop: usize,
}

impl<'a> Line<'a> {
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.source[self.start..self.stop]
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            "{:>5} | {}\n",
            "      | {:start$}{:^<len$}\n"),
               self.line_number, self.source,
               "", "", start = self.start, len = self.stop - self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &'static str = concat!(
        "A is: System;\n",
        "A consumes: B;\r\n",
        "A produces: C;\n");

    #[test]
    fn as_str() {
        let lexeme = Lexeme::new(SOURCE, 16, 25);
        assert_eq!("consumes:", lexeme.as_str());
    }

    #[test]
    fn display() {
        {
            let lexeme = Lexeme::new("X", 0, 1);
            assert_eq!(format!("{}", lexeme).as_str(), concat!(
                "      |\n",
                "    1 | X\n",
                "      | ^\n"));
        }
        {
            let lexeme = Lexeme::new(SOURCE, 0, 1);
            assert_eq!(format!("{}", lexeme).as_str(), concat!(
                "      |\n",
                "    1 | A is: System;\n",
                "      | ^\n"));
        }
        {
            let lexeme = Lexeme::new(SOURCE, 16, 25);
            assert_eq!(format!("{}", lexeme).as_str(), concat!(
                "      |\n",
                "    2 | A consumes: B;\n",
                "      |   ^^^^^^^^^\n"));
        }
        {
            let lexeme = Lexeme::new(SOURCE, 30, 44);
            assert_eq!(format!("{}", lexeme).as_str(), concat!(
                "      |\n",
                "    3 | A produces: C;\n",
                "      | ^^^^^^^^^^^^^^\n"));
        }
        {
            let lexeme = Lexeme::new(SOURCE, 16, 41);
            assert_eq!(format!("{}", lexeme).as_str(), concat!(
                "      |\n",
                "    2 | A consumes: B;\n",
                "      |   ^^^^^^^^^^^^\n",
                "    3 | A produces: C;\n",
                "      | ^^^^^^^^^^^\n"));
        }
    }
}
