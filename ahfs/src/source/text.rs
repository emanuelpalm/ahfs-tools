use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use super::{Range, Region};

/// A named source code text.
#[derive(Debug, Eq, PartialEq)]
pub struct Text {
    name: Box<str>,
    body: Box<str>,
}

impl Text {
    /// Creates new `Text` instance from given source `name` and `body`.
    #[inline]
    pub fn new<N, B>(name: N, body: B) -> Self
        where N: Into<Box<str>>,
              B: Into<Box<str>>,
    {
        Text { name: name.into(), body: body.into() }
    }

    /// Reads contents of file at `path` into a source `Text`.
    pub fn read<P>(path: P) -> io::Result<Text>
        where P: Into<PathBuf>
    {
        let path = path.into()
            .into_os_string()
            .into_string()
            .map_err(|path| io::Error::new(
                io::ErrorKind::Other,
                format!("Path not valid unicode {}", path.to_string_lossy())
            ))?;
        let body = {
            let mut file = fs::File::open(&path)?;
            let capacity = file.metadata()
                .map(|metadata| metadata.len() as usize + 1)
                .unwrap_or(0);
            let mut string = String::with_capacity(capacity);
            file.read_to_string(&mut string)?;
            string
        };
        Ok(Self::new(path, body))
    }

    /// `Text` name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// `Text` body.
    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Creates new `Text` containing copy of given `range` within this `Text`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn extract<R>(&self, range: R) -> Option<Text>
        where R: Into<Range>,
    {
        self.get(range).map(|body| Self::new(self.name(), body.as_str()))
    }

    /// Gets [`Region`](struct.Region.html) representing given `range` within
    /// this `Text`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn get<'a, R>(&'a self, range: R) -> Option<Region<'a>>
        where R: Into<Range>
    {
        let range = range.into();
        if range.start > self.body.len() || range.end > self.body.len() {
            return None;
        }
        Some(unsafe { Region::new(self, range) })
    }
}