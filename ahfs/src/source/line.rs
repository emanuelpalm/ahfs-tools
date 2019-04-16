use source::Range;
use std::fmt;

/// A source code line touching some significant range of characters.
pub struct Line<'a> {
    text: &'a str,
    number: usize,
    range: Range,
}

impl<'a> Line<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of `text`.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn new(text: &'a str, number: usize, range: Range) -> Self {
        Line { text, number, range }
    }

    /// Gets region line as `str`.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.text
    }

    /// `Line` number.
    #[inline]
    pub fn number(&self) -> usize {
        self.number
    }

    /// Bounds of significant region within `Line` as `str`.
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            "{:>5} | ", str_color!(none: "{}"), "\n",
            "      | ", str_color!( red: "{:start$}{:^<len$}"), "\n"),
               self.number, self.text, "", "",
               start = self.range.start,
               len = (self.range.end - self.range.start).max(1))
    }
}
