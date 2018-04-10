use std::fmt;
use std::isize;
use std::mem;
use super::LexemeKind;

/// Identifies a typed region within some source string.
pub struct Lexeme<'a, K = LexemeKind> {
    kind: K,
    region: &'a str,
}

impl<'a, K> Lexeme<'a, K> {
    #[inline]
    pub fn new(kind: K, region: &'a str) -> Self {
        Lexeme { kind, region }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.region
    }

    #[inline]
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// If this `Lexeme` originated from given `source`, get its `start..end`
    /// range within that source.
    #[inline]
    pub fn range_in(&self, source: &'a str) -> Option<(usize, usize)> {
        let region_start: usize = unsafe {
            mem::transmute(self.region.as_ptr())
        };
        let mut source_start: usize = unsafe {
            mem::transmute(source.as_ptr())
        };

        let region_end = region_start + self.region.len();
        let source_end = source_start + source.len();
        if region_start < source_start || region_end > source_end {
            return None;
        }

        let start = region_start - source_start;
        let end = start + self.region.len();
        Some((start, end))
    }

    /// Writes human-readable representation of `Lexeme` to given `writer` using
    /// provided `source` string as context.
    pub fn fmt<W>(&self, mut writer: W, mut source: &'a str) -> fmt::Result
        where W: fmt::Write,
    {
        let (start, end) = if let Some((start, end)) = self.range_in(source) {
            writeln!(writer, "      |")?;
            (start, end)
        } else {
            source = self.region;
            writeln!(writer, "      | !!")?;
            (0, source.len())
        };
        for (i, line) in Lines::new(source, start, end).enumerate() {
            if i < 2 {
                write!(writer, "{}", line)?;
            } else {
                writeln!(writer, "     ...")?;
                break;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn fmt_string(&self, source: &'a str) -> Result<String, fmt::Error> {
        let mut out = String::new();
        self.fmt(&mut out, source)?;
        Ok(out)
    }

    #[inline]
    pub fn repackage<L: fmt::Debug>(&self, kind: L) -> Lexeme<'a, L> {
        Lexeme { kind, region: self.region }
    }
}

impl<'a, K: Clone> Clone for Lexeme<'a, K> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.kind.clone(), self.region)
    }
}

impl<'a, K: Copy> Copy for Lexeme<'a, K> {}

impl<'a, K: fmt::Debug> fmt::Debug for Lexeme<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexeme {{ kind: {:?}, region: {:?} }}",
               self.kind, self.region)
    }
}

impl<'a> From<Lexeme<'a>> for Lexeme<'a, ()> {
    #[inline]
    fn from(lexeme: Lexeme<'a>) -> Self {
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
            "      | \x1b[31m{:start$}{:^<len$}\x1b[0m\n"),
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
        let format = |source: &str, start: usize, end: usize| {
            let lexeme = Lexeme::new((), &source[start..end]);
            lexeme.fmt_string(source).unwrap()
        };

        {
            let out = Lexeme::new((), "B").fmt_string(SOURCE).unwrap();
            assert_eq!(out.as_str(), concat!(
                "      | !!\n",
                "    1 | B\n",
                "      | \x1b[31m^\x1b[0m\n"));
        }
        {
            let out = format("X", 0, 1);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    1 | X\n",
                "      | \x1b[31m^\x1b[0m\n"));
        }
        {
            let out = format(SOURCE, 0, 1);
            assert_eq!(out, concat!(
                "      |\n",
                "    1 | A type System;\n",
                "      | \x1b[31m^\x1b[0m\n"));
        }
        {
            let out = format(SOURCE, 17, 25);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    2 | A consumes B;\n",
                "      | \x1b[31m  ^^^^^^^^\x1b[0m\n"));
        }
        {
            let out = format(SOURCE, 30, 42);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    3 | A produces C;\n",
                "      | \x1b[31m^^^^^^^^^^^^\x1b[0m\n"));
        }
        {
            let out = format(SOURCE, 17, 40);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    2 | A consumes B;\n",
                "      | \x1b[31m  ^^^^^^^^^^^\x1b[0m\n",
                "    3 | A produces C;\n",
                "      | \x1b[31m^^^^^^^^^^\x1b[0m\n"));
        }
        {
            let out = format(SOURCE, 7, 40);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    1 | A type System;\n",
                "      | \x1b[31m       ^^^^^^^\x1b[0m\n",
                "    2 | A consumes B;\n",
                "      | \x1b[31m^^^^^^^^^^^^^\x1b[0m\n",
                "     ...\n"));
        }
        {
            let out = format(SOURCE, 42, 42);
            assert_eq!(out.as_str(), concat!(
                "      |\n",
                "    3 | A produces C;\n",
                "      | \x1b[31m            ^\x1b[0m\n"));
        }
    }
}
