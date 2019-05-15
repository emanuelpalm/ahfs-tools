use crate::{Range, Span, Source};
use std::fmt;

/// Owned part of some original [`Source`][txt] containing a significant range
/// of characters.
///
/// [txt]: struct.Source.html
#[derive(Debug)]
pub struct Excerpt {
    /// Source excerpt.
    pub source: Source,

    /// Number of first line in source excerpt.
    pub line_number: usize,

    /// Range of significant characters in source excerpt.
    pub range: Range,
}

impl Excerpt {
    #[inline]
    pub fn as_span(&self) -> Span {
        Span {
            source: &self.source,
            range: self.range,
        }
    }
}

impl Default for Excerpt {
    fn default() -> Self {
        Excerpt {
            source: Source { name: "".into(), body: "".into() },
            line_number: 0,
            range: (0..0).into(),
        }
    }
}

impl fmt::Display for Excerpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_span(), f)
    }
}
