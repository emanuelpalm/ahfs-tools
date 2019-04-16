use source::{Line, Range};
use std::cmp::Ord;

/// An iterator over a set of source code [`Line`s][lin] containing a
/// significant range of characters.
///
/// [lin]: struct.Line.html
pub struct LineIter<'a> {
    text: &'a str,
    line_number: usize,
    range: Range,
}

impl<'a> LineIter<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of `text`.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new(text: &'a str, line_number: usize, range: Range) -> Self {
        LineIter { text, line_number, range }
    }

    /// Text string iterated over by `LineIter`.
    ///
    /// Note that the string shrinks each time a line is read.
    #[inline]
    pub fn text(&self) -> &str {
        self.text
    }

    /// Current line number.
    ///
    /// Note that the line number us incremented each time a line is read.
    #[inline]
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Significant range of characters within `text`.
    ///
    /// Note that the range is adjusted each time a line is read.
    #[inline]
    pub fn range(&self) -> &Range {
        &self.range
    }
}

impl<'a> Clone for LineIter<'a> {
    fn clone(&self) -> Self {
        LineIter {
            text: self.text,
            line_number: self.line_number,
            range: self.range.clone(),
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.len() == 0 {
            return None;
        }

        let (mut text_len, mut text_skip) = self.text.find('\n')
            .map(|index| (index, 1))
            .unwrap_or((self.text.len(), 0));

        if text_len > 0 && self.text.as_bytes()[text_len - 1] == b'\r' {
            text_len -= 1;
            text_skip += 1;
        }

        let text = &self.text[..text_len];
        let number = self.line_number;
        let range = Range::new(self.range.start, self.range.end.min(text.len()));

        text_len += text_skip;

        self.text = &self.text[text_len..];
        self.line_number += 1;
        self.range = Range::new(
            self.range.start.saturating_sub(text_len),
            self.range.end.saturating_sub(text_len)
        );

        Some(unsafe { Line::new(text, number, range) })
    }
}