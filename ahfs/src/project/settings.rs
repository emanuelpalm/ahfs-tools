use crate::error::Result;
use crate::meta;
use crate::project::{Error, Version};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// AHFS Project settings.
#[derive(Debug)]
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
        let file = format!("ahfs_version: {}\n", meta::VERSION_STR);

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
        let file = fs::read_to_string(path.clone())?;

        let entries = file.split("\n")
            .map(|line| {
                let pair: Vec<&str> = line.splitn(2, ":").collect();
                (
                    pair.get(0).map(|key| key.trim()).unwrap_or(""),
                    pair.get(1).map(|value| value.trim()).unwrap_or("")
                )
            })
            .fold(HashMap::new(), |mut map, (key, value)| {
                if key.len() > 0 {
                    map.insert(key, value);
                }
                map
            });

        let version_raw = entries
            .get("ahfs_version")
            .ok_or(Error::AhfsVersionMissing)?;

        let version = Version::parse(version_raw)
            .ok_or_else(|| Error::AhfsVersionInvalid {
                version: version_raw.to_string(),
            })?;

        if version.major() != meta::VERSION_MAJOR || version.minor() > meta::VERSION_MINOR {
            return Err(Box::new(Error::AhfsVersionIncompatible {
                version: version_raw.to_string(),
            }));
        }

        Ok(Settings {
            path: Path::new(".").into(),
            ahfs_version: version,
        })
    }

    /// Project AHFS version compatibility.
    #[inline]
    pub fn ahfs_version(&self) -> &Version {
        &self.ahfs_version
    }
}