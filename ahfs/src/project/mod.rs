//! Arrowhead specification project management.
//!
//! This module contains tools useful for managing a folder containing a
//! specification project.

mod error;
mod settings;
mod version;

pub use self::error::Error;
pub use self::settings::Settings;
pub use self::version::Version;

use crate::error::Result;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents an AHFS project.
#[derive(Debug)]
pub struct Project {
    root: Box<Path>,
    settings: Box<Settings>,
    target: Box<Path>,
}

impl Project {
    /// Attempts to create new AHFS project at given `path`.
    ///
    /// Concretely, tries to create an `".ahfs"` folder inside `path` and fill
    /// it with default project settings.
    pub fn create<P>(path: P) -> Result<Project>
        where P: Into<PathBuf>
    {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let settings = Settings::create(path.join(".ahfs"))?;
        let target = path.join("target");

        Ok(Project {
            root: path.into(),
            settings: settings.into(),
            target: target.into(),
        })
    }

    /// Attempts to locate AHFS project by looking inside `path` and all of its
    /// parent directories.
    pub fn locate<P>(path: P) -> Result<Project>
        where P: Into<PathBuf>
    {
        let mut path = path.into();
        loop {
            path.push(".ahfs");
            let is_file = path.is_file();
            path.pop();
            if is_file {
                break;
            }
            if !path.pop() {
                let err: io::Error = io::ErrorKind::NotFound.into();
                return Err(err.into());
            }
        }
        let settings = Settings::read(&path.join(".ahfs"))?;
        let target = path.join("target");

        Ok(Project {
            root: path.into(),
            settings: settings.into(),
            target: target.into(),
        })
    }

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
                    },
                    t @ _ if t.is_file() => {}
                    _ => { continue; }
                }
                let path = entry.path();
                if path.extension().unwrap_or_default() != "ahfs" {
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

    /// Project settings.
    #[inline]
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    #[inline]
    pub fn target(&self) -> &Path {
        &self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta;

    #[test]
    fn create_and_locate() {
        let path: PathBuf = ".test-project-folder-0".into();
        let version = Version::new(
            meta::VERSION_MAJOR,
            meta::VERSION_MINOR,
            meta::VERSION_PATCH);
        let version_create = {
            let project = Project::create(path.clone()).unwrap();
            *project.settings().ahfs_version()
        };
        let version_locate = {
            let project = Project::locate(path.clone()).unwrap();
            *project.settings().ahfs_version()
        };
        fs::remove_dir_all(path).unwrap();

        assert_eq!(version, version_create);
        assert_eq!(version, version_locate);
    }
}
