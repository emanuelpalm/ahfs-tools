use std::fmt;
use std::ops;

/// An integer range.
#[derive(Copy, Clone)]
pub struct Range {
    /// Start of range.
    pub start: usize,

    /// First integer after end of range.
    pub end: usize,
}

impl Range {
    /// Number of elements in range.
    #[inline]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl From<ops::Range<usize>> for Range {
    #[inline]
    fn from(range: ops::Range<usize>) -> Self {
        Range { start: range.start, end: range.end }
    }
}

impl From<Range> for ops::Range<usize> {
    #[inline]
    fn from(range: Range) -> Self {
        ops::Range { start: range.start, end: range.end }
    }
}