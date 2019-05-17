//! Arrowhead specification project management.
//!
//! This module contains tools useful for managing a folder containing a
//! specification project.

pub mod parser;

mod configuration;

pub use self::configuration::Configuration;

use crate::error::Result;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Default project file name.
const PROJECT_FILE: &'static str = "project.txt";

// File extension of project source files.
const SOURCE_EXTENSION: &'static str = "ahfs";

/// An Arrowhead Framework specification project.
#[derive(Debug)]
pub struct Project {
    root: Box<Path>,
    configuration: Configuration,
}

impl Project {
    /// Attempt to create new specification project with `name` at given `path`.
    ///
    /// Concretely, tries to create an `"project.txt"` folder inside `path`
    /// and fill it with default options.
    pub fn create<N, P>(name: N, path: P) -> Result<Project>
        where N: AsRef<str>,
              P: Into<PathBuf>,
    {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let configuration = Configuration::new(name.as_ref());
        configuration.write_to(path.join(PROJECT_FILE))?;

        Ok(Project {
            root: path.into(),
            configuration,
        })
    }

    /// Attempt to locate specification project by looking inside `path` and
    /// all of its parent directories.
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

        let configuration = Configuration::read_at(&path)?;
        path.pop();

        Ok(Project {
            root: path.into(),
            configuration,
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

    /// Project options, as specified in `project.txt` file in project root.
    #[inline]
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    /// Target output folder.
    #[inline]
    pub fn target(&self) -> PathBuf {
        let mut buf: PathBuf = self.root().into();
        buf.push("target");
        buf
    }
}
