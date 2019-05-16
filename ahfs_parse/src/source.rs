use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

/// A named source code text.
#[derive(Debug)]
pub struct Source {
    pub name: Box<str>,
    pub body: Box<str>,
}

impl Source {
    /// Reads contents of file at `path` into a `Source`.
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
        Ok(Source { name: path.into(), body: body.into() })
    }
}
