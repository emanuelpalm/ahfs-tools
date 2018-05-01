use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use super::{Range, Region};

/// A named source code text.
#[derive(Debug, Eq, PartialEq)]
pub struct Text<'a> {
    name: &'a str,
    body: Box<str>,
}

impl<'a> Text<'a> {
    /// Creates new `Text` instance from given source `name` and `body`.
    #[inline]
    pub fn new<N, B>(name: N, body: B) -> Self
        where N: Into<&'a str>,
              B: Into<Box<str>>,
    {
        Text { name: name.into(), body: body.into() }
    }

    /// Reads contents of file at `path` into a source `Text`.
    pub fn read(path: &'a Path) -> io::Result<Text<'a>> {
        let name = path.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Path not valid unicode {}", path.to_string_lossy()),
            )
        })?;
        let mut file = fs::File::open(path)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;
        Ok(Self::new(name, body))
    }

    /// `Text` name.
    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

    /// `Text` body.
    #[inline]
    pub fn body(&self) -> &str {
        &self.body
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
}