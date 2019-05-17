use std::fs;
use std::io;
use std::path::Path;

/// An owned source code text.
#[derive(Debug)]
pub struct Text {
    /// Name of text.
    pub name: Box<str>,

    /// Concrete text contents.
    pub body: Box<str>,
}

impl Text {
    /// Attempt to read contents of file at `path` into a new `Text`.
    pub fn read_at<P>(path: P) -> io::Result<Text>
        where P: AsRef<Path>
    {
        let path = path.as_ref();
        let name = path.to_string_lossy();
        let body = fs::read_to_string(path)?;

        Ok(Text { name: name.to_string().into(), body: body.into() })
    }
}
