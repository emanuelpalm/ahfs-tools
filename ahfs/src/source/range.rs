use std::fmt;
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

    pub fn as_ops_range(self) -> ops::Range<usize> {
        ops::Range { start: self.start, end: self.end }
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}