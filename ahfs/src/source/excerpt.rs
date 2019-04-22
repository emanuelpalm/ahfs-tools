use crate::source::{LineIter, Lines, Range, Span, Source};
use std::fmt;

/// Owned part of some original [`Text`][txt] containing a significant range of
/// characters.
///
/// [txt]: struct.Text.html
#[derive(Debug)]
pub struct Excerpt {
    line_number: usize,
    source: Source,
    range: Range,
}

impl Excerpt {
    /// Number of line in original [`Text`][txt] at which `Excerpt` text begins.
    ///
    /// [txt]: struct.Text.html
    #[inline]
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Significant [`Range`][ran] of bytes within `Excerpt` source.
    ///
    /// [ran]: type.Range.html
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }

    /// `Excerpt` source text.
    #[inline]
    pub fn source(&self) -> &Source {
        &self.source
    }
}

impl Default for Excerpt {
    fn default() -> Self {
        Excerpt {
            line_number: 1,
            source: Source::new("", ""),
            range: Range::new(0, 0),
        }
    }
}

impl fmt::Display for Excerpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Lines::fmt(self, f, self.source.name())
    }
}

impl<'a> From<Span<'a>> for Excerpt {
    #[inline]
    fn from(region: Span<'a>) -> Self {
        Self::from(&region)
    }
}

impl<'a, 'b> From<&'a Span<'b>> for Excerpt {
    fn from(span: &'a Span<'b>) -> Self {
        let lines = span.lines();

        let source = Source::new(span.source().name(), lines.text());
        let line_number = lines.line_number();
        let range = lines.range().clone();

        Excerpt { source, line_number, range }
    }
}

impl Lines for Excerpt {
    fn lines(&self) -> LineIter {
        let text = self.source().body();
        let line_number = self.line_number();
        let range = self.range().clone();
        unsafe { LineIter::new(text, line_number, range) }
    }
}