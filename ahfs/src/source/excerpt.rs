use source::{LineIter, Lines, Range, Region, Text};
use std::fmt;

/// Owned part of some original [`Text`][txt] containing a significant range of
/// characters.
///
/// [txt]: struct.Text.html
#[derive(Debug)]
pub struct Excerpt {
    text: Text,
    line_number: usize,
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

    /// Significant [`Range`][ran] of bytes within `Excerpt` text.
    ///
    /// [ran]: type.Range.html
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }

    /// `Excerpt` text.
    #[inline]
    pub fn text(&self) -> &Text {
        &self.text
    }
}

impl fmt::Display for Excerpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Lines::fmt(self, f, self.text.name())
    }
}

impl<'a> From<Region<'a>> for Excerpt {
    #[inline]
    fn from(region: Region<'a>) -> Self {
        Self::from(&region)
    }
}

impl<'a, 'b> From<&'a Region<'b>> for Excerpt {
    fn from(region: &'a Region<'b>) -> Self {
        let lines = region.lines();

        let text = Text::new(region.text().name(), lines.text());
        let line_number = lines.line_number();
        let range = lines.range().clone();

        Excerpt { text, line_number, range }
    }
}

impl Lines for Excerpt {
    fn lines(&self) -> LineIter {
        let text = self.text().body();
        let line_number = self.line_number();
        let range = self.range().clone();
        unsafe { LineIter::new(text, line_number, range) }
    }
}