//! Arrowhead specification project management.
//!
//! This module contains tools useful for managing a folder containing a
//! specification project.

mod options;

pub use self::options::Options;

use crate::error::Result;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Default project file name.
const PROJECT_FILE: &'static str = "project.toml";

// File extension of project source files.
const SOURCE_EXTENSION: &'static str = "ahfs";

/// Represents an AHFS project.
#[derive(Debug)]
pub struct Project {
    root: Box<Path>,
    options: Options,
}

impl Project {
    /// Attempts to create new AHFS project with `name` at given `path`.
    ///
    /// Concretely, tries to create an `"project.toml"` folder inside `path`
    /// and fill it with default options.
    pub fn create<N, P>(name: N, path: P) -> Result<Project>
        where N: AsRef<str>,
              P: Into<PathBuf>,
    {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let options = Options::new(name.as_ref());
        options.write_to(path.join(PROJECT_FILE))?;

        Ok(Project {
            root: path.into(),
            options,
        })
    }

    /// Attempts to locate AHFS project by looking inside `path` and all of its
    /// parent directories.
    pub fn locate<P>(path: P) -> Result<Project>
        where P: Into<PathBuf>
    {
        let mut path = path.into();
        loop {
            path.push(PROJECT_FILE);
            if path.is_file() {
                break;
            }
            path.pop();
            if !path.pop() {
                let err: io::Error = io::ErrorKind::NotFound.into();
                return Err(err.into());
            }
        }

        let options = Options::read_at(&path)?;
        path.pop();

        Ok(Project {
            root: path.into(),
            options,
        })
    }

    /// Assembles list of all project source files.
    pub fn files(&self) -> Result<Box<[PathBuf]>> {
        let mut files = Vec::new();
        files_inner(self.root(), &mut files)?;
        return Ok(files.into_boxed_slice());

        fn files_inner(dir: &Path, files: &mut Vec<PathBuf>) -> Result {
            for entry in dir.read_dir()? {
                let entry = entry?;
                match entry.file_type()? {
                    t @ _ if t.is_dir() => {
                        files_inner(&entry.path(), files)?;
                        continue;
                    }
                    t @ _ if t.is_file() => {}
                    _ => { continue; }
                }
                let path = entry.path();
                if path.extension().unwrap_or_default() != SOURCE_EXTENSION {
                    continue;
                }
                files.push(path);
            }
            Ok(())
        }
    }

    /// Project root folder.
    #[inline]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Project options, as specified in `project.toml` file in project root.
    #[inline]
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Target output folder.
    #[inline]
    pub fn target(&self) -> PathBuf {
        let mut buf: PathBuf = self.root().into();
        buf.push(self.options()
            .project()
            .out_dir()
            .unwrap_or_else(|| "target".as_ref()));
        buf
    }
}
