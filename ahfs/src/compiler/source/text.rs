use super::{Range, Region};

/// A named source code text.
#[derive(Debug, Eq, PartialEq)]
pub struct Text<'a> {
    name: &'a str,
    body: &'a str,
}

impl<'a> Text<'a> {
    /// Creates new `Text` instance from given source `name` and `body`.
    #[inline]
    pub fn new<N, B>(name: N, body: B) -> Self
        where N: Into<&'a str>,
              B: Into<&'a str>,
    {
        Text { name: name.into(), body: body.into() }
    }

    /// `Text` name.
    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// `Text` body.
    #[inline]
    pub fn body(&self) -> &'a str {
        self.body
    }

    /// Gets `&'a str` representing given `range` within this `Text`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn get<R>(&'a self, range: R) -> Option<&'a str>
        where R: Into<Range>
    {
        self.get_region(range).map(|region| region.as_str())
    }

    /// Gets [`Region`](struct.Region.html) representing given `range` within
    /// this `Text`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn get_region<R>(&'a self, range: R) -> Option<Region<'a>>
        where R: Into<Range>
    {
        let range = range.into();
        if range.start > self.body.len() || range.end > self.body.len() {
            return None;
        }
        Some(unsafe { Region::new(self, range) })
    }

    /// Constructs [`Region`](struct.Region.html) representing first byte after
    /// end of this `Text`.
    pub fn end_region(&'a self) -> Region<'a> {
        unsafe { Region::new(self, self.body.len()..self.body.len()) }
    }
}