mod project;

pub use self::project::OptionsProject;

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

/// AHFS Project settings.
#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    project: OptionsProject,
}

impl Options {
    /// Attempts to create `Options` file at given `path`, which is also
    /// returned.
    pub fn new<N>(name: N) -> Self
        where N: AsRef<str>,
    {
        Options {
            project: OptionsProject::new(name.as_ref()),
        }
    }

    /// Reference to configured project options.
    #[inline]
    pub fn project(&self) -> &OptionsProject {
        &self.project
    }

    /// Attempts to read `Options` from file at given `path`.
    #[inline]
    pub fn read_at<P>(path: P) -> Result<Options>
        where P: AsRef<Path>,
    {
        let data = fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    /// Writes `Options` to file at given `path`.
    pub fn write_to<P>(&self, path: P) -> Result
        where P: AsRef<Path>,
    {
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.as_ref())?
            .write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }
}
