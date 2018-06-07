use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use super::{Error, Version};
use ::error::Result;
use ::graph::{Graph, Query};
use ::meta;
use ::source::Source;

/// AHFS Project settings.
pub struct Settings {
    path: Box<Path>,
    ahfs_version: Version,
}

impl Settings {
    /// Attempts to create `Settings` file at given `path`, which is also
    /// returned.
    pub fn create<P>(path: P) -> Result<Settings>
        where P: Into<PathBuf>
    {
        let path = path.into();
        let ahfs_version = Version::new(
            meta::VERSION_MAJOR,
            meta::VERSION_MINOR,
            meta::VERSION_PATCH);
        let file = format!("Project ahfs.version {};\n", meta::VERSION_STR);
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?
            .write_all(file.as_ref())?;
        Ok(Settings { path: path.into(), ahfs_version })
    }

    /// Attempts to read `Settings` from file at given `path`.
    pub fn read<P>(path: P) -> Result<Settings>
        where P: Into<PathBuf>,
    {
        let path = path.into();
        let source = Source::read_file(path.clone())?;
        let triples = ::parser::parse(&source)?;

        let ahfs_version_obj = triples.query()
            .subject("Project")
            .predicate("ahfs.version")
            .next()
            .ok_or(Error::AhfsVersionMissing)?
            .object();
        let ahfs_version_opt = Version::parse(ahfs_version_obj.as_str());
        let ahfs_version = match ahfs_version_opt {
            Some(v) => v,
            None => {
                return Err(Box::new(Error::AhfsVersionInvalid {
                    excerpt: ahfs_version_obj.into()
                }));
            }
        };
        if ahfs_version.major() != ::meta::VERSION_MAJOR ||
            ahfs_version.minor() > ::meta::VERSION_MINOR {
            return Err(Box::new(Error::AhfsVersionIncompatible {
                excerpt: ahfs_version_obj.into(),
            }));
        }

        Ok(Settings {
            path: path.into(),
            ahfs_version: ahfs_version.into(),
        })
    }

    /// Project AHFS version compatibility.
    #[inline]
    pub fn ahfs_version(&self) -> &Version {
        &self.ahfs_version
    }
}