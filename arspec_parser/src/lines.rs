use arspec_macro::color;
use crate::Range;
use std::fmt;

/// An iterator over zero or more lines of some source [`text`][txt].
///
/// [txt]: struct.Text.html
pub struct Lines<'a> {
    /// Source text iterated over.
    pub source: &'a str,

    /// Line number at which `source` text begins.
    pub number: usize,

    /// Any significant region of characters within `source` text.
    pub range: Option<Range>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Line<'a>> {
        if self.source.len() == 0 {
            return None;
        }

        let (mut line_len, mut line_skip) = self.source.find('\n')
            .map(|index| (index, 1))
            .unwrap_or((self.source.len(), 0));

        if line_len > 0 && self.source.as_bytes()[line_len - 1] == b'\r' {
            line_len -= 1;
            line_skip += 1;
        }

        let source = &self.source[..line_len];
        let number = self.number;
        let range = self.range.and_then(|range| {
            let range = Range {
                start: range.start,
                end: range.end.min(source.len()),
            };
            if range.len() > 0 {
                Some(range)
            } else {
                None
            }
        });

        line_skip += line_len;

        self.source = &self.source[line_skip..];
        self.number += 1;
        self.range = self.range.map(|range| Range {
            start: range.start.saturating_sub(line_skip),
            end: range.end.saturating_sub(line_skip),
        });

        Some(Line { source, number, range })
    }
}

/// A source [`text`][txt] line that may contain a significant range of
/// characters.
///
/// [txt]: struct.Text.html
pub struct Line<'a> {
    /// Line of source text.
    pub source: &'a str,

    /// Line number.
    pub number: usize,

    /// Any significant region of characters in `source` text.
    pub range: Option<Range>,
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            concat!( "{:>5} | ", color!(_: "{}"), "\n"),
            self.number, self.source
        )?;
        if let Some(range) = self.range {
            write!(
                f,
                concat!("      | ", color!(r: "{:start$}{:^<len$}"), "\n"),
                "", "",
                start = range.start,
                len = (range.end.saturating_sub(range.start)).max(1)
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use arspec_macro::color;
    use std::fmt::Write;
    use std::ops;
    use super::*;

    #[test]
    fn display() {
        let source = concat!(
            "A type System;\n",
            "A consumes B;\r\n",
            "A produces C;\n",
        );

        let get = |range: ops::Range<usize>| {
            let lines = Lines {
                source,
                number: 1,
                range: Some(range.into())
            };
            let mut buffer = String::new();
            for line in lines {
                write!(buffer, "{}", line).unwrap();
            }
            buffer
        };

        assert_eq!(get(0..1).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "      | ", color!(r: "^"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n"));

        assert_eq!(get(17..25).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "  ^^^^^^^^"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n"));

        assert_eq!(get(30..42).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^^^"), "\n"));

        assert_eq!(get(17..40).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "  ^^^^^^^^^^^"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^"), "\n"));

        assert_eq!(get(7..40).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "      | ", color!(r: "       ^^^^^^^"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^^^^"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^"), "\n"));

        assert_eq!(get(42..42).as_str(), concat!(
            "    1 | ", color!(_: "A type System;"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n"));
    }
}