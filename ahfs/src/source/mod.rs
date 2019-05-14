//! Various types and utilities related to managing source code texts.

mod excerpt;
mod line;
mod lines;
mod line_iter;
mod range;
mod span;

pub use self::excerpt::Excerpt;
pub use self::line::Line;
pub use self::lines::Lines;
pub use self::line_iter::LineIter;
pub use self::range::Range;
pub use self::span::Span;

use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

/// A named source code text.
#[derive(Debug, Eq, PartialEq)]
pub struct Source {
    name: Box<str>,
    body: Box<str>,
}

impl Source {
    /// Creates new `Source` instance from given source `name` and `body`.
    #[inline]
    pub fn new<N, B>(name: N, body: B) -> Self
        where N: Into<Box<str>>,
              B: Into<Box<str>>,
    {
        Source { name: name.into(), body: body.into() }
    }

    /// Reads contents of file at `path` into a source `Source`.
    pub fn read<P>(path: P) -> io::Result<Source>
        where P: Into<PathBuf>
    {
        let path = path.into()
            .into_os_string()
            .into_string()
            .map_err(|path| io::Error::new(
                io::ErrorKind::Other,
                format!("Path not valid unicode {}", path.to_string_lossy()),
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

    /// `Source` name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// `Source` body.
    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Creates new `Source` containing copy of given `range` within this
    /// `Source`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn extract<R>(&self, range: R) -> Option<Source>
        where R: Into<Range>,
    {
        self.get(range).map(|body| Self::new(self.name(), body.as_str()))
    }

    /// Gets [`Region`](struct.Region.html) representing given `range` within
    /// this `Source`.
    ///
    /// Returns `None` if `range` is out of bounds.
    pub fn get<R>(&self, range: R) -> Option<Span>
        where R: Into<Range>
    {
        let range = range.into();
        if range.start > self.body.len() || range.end > self.body.len() {
            return None;
        }
        Some(unsafe { Span::new(self, range) })
    }
}

#[cfg(test)]
mod tests {
    use ahfs_macro::color;
    use super::*;

    #[test]
    fn display() {
        let source = Source::new("alpha.ahs", concat!(
            "A type System;\n",
            "A consumes B;\r\n",
            "A produces C;\n",
        ));
        let get = |range: Range| {
            format!("{}", source.get(range).unwrap())
        };

        assert_eq!(get(Range::new(0, 1)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    1 | ", color!(_: "A type System;"), "\n",
            "      | ", color!(r: "^"), "\n"));

        assert_eq!(get(Range::new(17, 25)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "  ^^^^^^^^"), "\n"));

        assert_eq!(get(Range::new(30, 42)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^^^"), "\n"));

        assert_eq!(get(Range::new(17, 40)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "  ^^^^^^^^^^^"), "\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^"), "\n"));

        assert_eq!(get(Range::new(7, 40)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    1 | ", color!(_: "A type System;"), "\n",
            "      | ", color!(r: "       ^^^^^^^"), "\n",
            "    2 | ", color!(_: "A consumes B;"), "\n",
            "      | ", color!(r: "^^^^^^^^^^^^^"), "\n",
            "     ...\n"));

        assert_eq!(get(Range::new(42, 42)).as_str(), concat!(
            "      : ", color!(b: "alpha.ahs"), "\n",
            "      |\n",
            "    3 | ", color!(_: "A produces C;"), "\n",
            "      | ", color!(r: "            ^"), "\n"));
    }
}