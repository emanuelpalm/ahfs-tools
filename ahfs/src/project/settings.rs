use std::path::{Path, PathBuf};
use super::{Error, Version};
use ::error::Result;
use ::graph::{Graph, Query};
use ::source::Source;

pub struct Settings {
    path: Box<Path>,
    ahfs_version: Version,
}

impl Settings {
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
}