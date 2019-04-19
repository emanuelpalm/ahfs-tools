use source::{LineIter, Lines, Range, Source};
use std::cmp;
use std::fmt;

/// Represents a significant region within a borrowed source code text.
#[derive(Clone)]
pub struct Span<'a> {
    source: &'a Source,
    range: Range,
}

impl<'a> Span<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of given
    /// [`text`](struct.Text.html).
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new(text: &'a Source, range: Range) -> Self {
        Span { source: text, range }
    }

    /// Gets string representing only significant range within this `Region`.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        unsafe {
            self.source.body().get_unchecked(self.range.as_ops_range())
        }
    }

    /// Byte range of this `Region` within its `text`.
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }

    /// [`Source`](struct.Source.html) in which `Region` is located.
    #[inline]
    pub fn source(&self) -> &'a Source {
        self.source
    }

    /// Creates new `Region` representing only end of this `Region`.
    #[inline]
    pub fn end(&self) -> Self {
        Span { source: self.source, range: Range::new(self.range.end, self.range.end) }
    }

    /// Connects this and given `other` `Region`, creating a new `Region` that
    /// contains both regions and all text between them.
    #[inline]
    pub fn connect(&self, other: Span<'a>) -> Self {
        Span {
            source: self.source,
            range: Range::new(
                cmp::min(self.range.start, other.range.start),
                cmp::max(self.range.end, other.range.end),
            ),
        }
    }
}

impl<'a> AsRef<str> for Span<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> fmt::Debug for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "Region {{ text: `{}` ({}), range: {:?} }}",
            &self.source.body()[self.range.as_ops_range()],
            self.source.name(),
            self.range.clone()
        )
    }
}

impl<'a> fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Lines::fmt(self, f, self.source.name())
    }
}

impl<'a> Lines for Span<'a> {
    fn lines(&self) -> LineIter {
        let body = self.source.body();

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

        let text = &body[start..end];
        let line_number = body[..start]
            .bytes()
            .filter(|b| *b == b'\n')
            .count() + 1;
        let range = Range::new(self.range.start - start, self.range.end - start);

        unsafe { LineIter::new(text, line_number, range) }
    }
}

impl<'a, 'b> PartialEq<&'a str> for Span<'b> {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        &self.as_str() == other
    }
}

impl<'a, 'b> PartialEq<Span<'a>> for &'b str {
    #[inline]
    fn eq(&self, other: &Span<'a>) -> bool {
        other == self
    }
}