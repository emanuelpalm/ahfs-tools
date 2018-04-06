use std::fmt;
use super::LexemeKind;

/// Identifies a typed region within some source string.
///
/// # Offsets
///
/// The `Lexeme` `start` and `end` offsets identify a bytes range including
/// `start` and excluding `end`.
///
/// # Detached
///
/// The `Lexeme` struct does not include an explicit reference to its source
/// string, meaning it must be kept track of some other way.
pub struct Lexeme<K = LexemeKind> {
    kind: K,
    start: usize,
    end: usize,
}

impl<K> Lexeme<K> {
    #[inline]
    pub fn new(kind: K, start: usize, end: usize) -> Self {
        Lexeme { kind, start, end }
    }

    #[inline]
    pub fn kind(&self) -> &K {
        &self.kind
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Extracts `Lexeme` string from given `source` string.
    ///
    /// # Panics
    ///
    /// If the `Lexeme` `start` and `end` offsets would be out of the `source`
    /// string bounds, or if `start` would be greater than `end`, the method
    /// panics.
    #[inline]
    pub fn extract<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }

    /// Writes human-readable representation of `Lexeme` to given `writer` using
    /// provided `source` string as context.
    ///
    /// # Panics
    ///
    /// If the `Lexeme` `start` and `end` offsets would be out of the `source`
    /// string bounds, or if `start` would be greater than `end`, the method
    /// panics.
    pub fn fmt_using<W>(&self, mut writer: W, source: &str) -> fmt::Result
        where W: fmt::Write,
    {
        write!(writer, "      |\n")?;
        for (i, line) in Lines::new(source, self.start, self.end).enumerate() {
            if i < 2 {
                write!(writer, "{}", line)?;
            } else {
                write!(writer, "     ...\n")?;
                break;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn fmt_string_using(&self, source: &str) -> Result<String, fmt::Error> {
        let mut out = String::new();
        self.fmt_using(&mut out, source)?;
        Ok(out)
    }

    #[inline]
    pub fn repackage<L: fmt::Debug>(&self, kind: L) -> Lexeme<L> {
        Lexeme { kind, start: self.start, end: self.end }
    }

    #[inline]
    pub fn shrink(mut self, from_start: usize, from_end: usize) -> Lexeme<K> {
        use std::cmp::Ord;

        self.start = self.start.saturating_add(from_start);
        self.end = self.end.saturating_sub(from_end).max(self.start);
        self
    }
}

impl<K: Clone> Clone for Lexeme<K> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.kind.clone(), self.start, self.end)
    }
}

impl<K: Copy> Copy for Lexeme<K> {}

impl<K: fmt::Debug> fmt::Debug for Lexeme<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexeme {{ kind: {:?}, start: {}, end: {} }}",
            self.kind, self.start, self.end)
    }
}

impl From<Lexeme> for Lexeme<()> {
    #[inline]
    fn from(lexeme: Lexeme) -> Self {
        lexeme.repackage(())
    }
}

struct Lines<'a> {
    source: &'a str,
    start: usize,
    end: usize,

    line_number: usize,
    offset: usize,
}

impl<'a> Lines<'a> {
    #[inline]
    fn new(source: &'a str, start: usize, end: usize) -> Self {
        // Find first newline before lexeme.
        let before = match source[..start].rfind('\n') {
            Some(index) => index + 1,
            None => 0,
        };

        // Find first newline after lexeme.
        let after = end + match source[end..].find('\n') {
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
            source: &source[before..after],
            line_number: source[..before]
                .bytes()
                .filter(|b| *b == b'\n')
                .count() + 1,
            start: start - before,
            end: end - before,
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
            Some(index) => (self.offset + index, false),
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

        // Determine end of lexeme within line.
        let end = if at_end { self.end } else { offset } - self.offset;

        // Forward internal offset and line_number.
        self.offset = offset + skip;
        self.line_number += 1;

        Some(Line { source, line_number, start, end })
    }
}

struct Line<'a> {
    source: &'a str,
    start: usize,
    end: usize,

    line_number: usize,
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cmp::Ord;

        write!(f, concat!(
            "{:>5} | {}\n",
            "      | {:start$}{:^<len$}\n"),
               self.line_number, self.source, "", "",
               start = self.start,
               len = (self.end - self.start).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &'static str = concat!(
        "A type System;\n",
        "A consumes B;\r\n",
        "A produces C;\n");

    #[test]
    fn display() {
        let format = |lexeme: Lexeme<_>, source: &str| {
            lexeme.fmt_string_using(source).unwrap()
        };

        {
            let out = format(Lexeme::new((), 0, 1), "X");
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    1 | X\n",
                "      | ^\n"));
        }
        {
            let out = format(Lexeme::new((), 0, 1), SOURCE);
            assert_eq!(out, concat!(
                "      |\n",
                "    1 | A type System;\n",
                "      | ^\n"));
        }
        {
            let out = format(Lexeme::new((), 17, 25), SOURCE);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    2 | A consumes B;\n",
                "      |   ^^^^^^^^\n"));
        }
        {
            let out = format(Lexeme::new((), 30, 42), SOURCE);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    3 | A produces C;\n",
                "      | ^^^^^^^^^^^^\n"));
        }
        {
            let out = format(Lexeme::new((), 17, 40), SOURCE);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    2 | A consumes B;\n",
                "      |   ^^^^^^^^^^^\n",
                "    3 | A produces C;\n",
                "      | ^^^^^^^^^^\n"));
        }
        {
            let out = format(Lexeme::new((), 7, 40), SOURCE);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    1 | A type System;\n",
                "      |        ^^^^^^^\n",
                "    2 | A consumes B;\n",
                "      | ^^^^^^^^^^^^^\n",
                "     ...\n"));
        }
        {
            let out = format(Lexeme::new((), 42, 42), SOURCE);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    3 | A produces C;\n",
                "      |             ^\n"));
        }
    }
}
