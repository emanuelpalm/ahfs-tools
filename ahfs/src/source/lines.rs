use std::cmp::Ord;
use super::{Line, Range};

/// An iterator over a set of source code lines containing a significant range
/// of characters.
pub struct Lines<'a> {
    text: &'a str,
    line_number: usize,
    range: Range,
}

impl<'a> Lines<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`range`](type.Range.html) is within valid UTF-8 bounds of `text`.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new(text: &'a str, range: Range) -> Self {
        let start = text[..range.start]
            .rfind('\n')
            .map(|start| start + 1)
            .unwrap_or(0);

        let end = text[range.end..]
            .find('\n')
            .map(|mut end| {
                end += range.end;
                if end > 0 && text.as_bytes()[end - 1] == b'\r' {
                    end -= 1;
                }
                end
            })
            .unwrap_or(range.end);

        Lines {
            text: &text[start..end],
            line_number: text[..start]
                .bytes()
                .filter(|b| *b == b'\n')
                .count() + 1,
            range: (range.start - start)..(range.end - start),
        }
    }
}

impl<'a> Clone for Lines<'a> {
    fn clone(&self) -> Self {
        Lines {
            text: self.text,
            line_number: self.line_number,
            range: self.range.clone(),
        }
    }
}

impl<'a> Iterator for Lines<'a> {
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
        let range = self.range.start..self.range.end.min(text.len());

        text_len += text_skip;

        self.text = &self.text[text_len..];
        self.line_number += 1;
        self.range = (self.range.start.saturating_sub(text_len))
            ..(self.range.end.saturating_sub(text_len));

        Some(unsafe { Line::new(text, number, range) })
    }
}