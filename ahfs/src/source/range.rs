use crate::source::Span;
use std::ops;

/// Refers to a range of bytes within some arbitrary `str`.
#[derive(Copy, Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    #[inline]
    pub fn as_ops_range(self) -> ops::Range<usize> {
        ops::Range { start: self.start, end: self.end }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl<'a> From<&Span<'a>> for Range {
    #[inline]
    fn from(span: &Span<'a>) -> Self {
        span.range().clone()
    }
}