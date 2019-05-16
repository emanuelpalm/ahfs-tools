use ahfs_macro::color;
use crate::{Excerpt, Lines, Range, Source};
use std::fmt;
use std::ops;

/// Represents a significant region within a borrowed source code text.
#[derive(Clone)]
pub struct Span<'a> {
    /// Source text referred to.
    pub source: &'a Source,

    /// Delimits significant region within `source`.
    pub range: Range,
}

impl<'a> Span<'a> {
    /// Gets string representing only significant region within this `Span`.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        let range: ops::Range<usize> = self.range.into();
        self.source.body.get(range).unwrap_or("")
    }

    /// Creates iterator over lines touched by significant region of this
    /// `Span`.
    pub fn lines(&self) -> Lines {
        let body = &self.source.body;

        let start = body[..self.range.start]
            .rfind('\n')
            .map(|start| start + 1)
            .unwrap_or(0);

        let end = body[self.range.end..]
            .find('\n')
            .map(|mut end| {
                end += self.range.end;
                if end > 0 && body.as_bytes()[end - 1] == b'\r' {
                    end -= 1;
                }
                end
            })
            .unwrap_or(self.range.end);

        let source = &body[start..end];
        let number = body[..start]
            .bytes()
            .filter(|b| *b == b'\n')
            .count() + 1;
        let range = Some(Range {
            start: self.range.start - start,
            end: self.range.end - start,
        });

        Lines { source: source, number, range }
    }

    /// Creates an owned `Excerpt` from this `Span`.
    pub fn to_excerpt(&self) -> Excerpt {
        let lines = self.lines();
        let offset = (self.source.body.as_ptr() as usize)
            .saturating_sub(lines.source.as_ptr() as usize);
        Excerpt {
            source: Source {
                name: self.source.name.clone(),
                body: lines.source.into(),
            },
            line_number: lines.number,
            range: Range {
                start: self.range.start.saturating_sub(offset),
                end: self.range.end.saturating_sub(offset),
            },
        }
    }
}

impl<'a> fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let range: ops::Range<usize> = self.range.into();
        write!(
            f, "Span {{ source: `{}` ({}), range: {:?} }}",
            &self.source.body.get(range).unwrap_or(""),
            self.source.name,
            self.range,
        )
    }
}

impl<'a> fmt::Display for Span<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            "      : ", color!(b: "{}"), "\n",
            "      |\n"), self.source.name)?;
        for line in self.lines() {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}
