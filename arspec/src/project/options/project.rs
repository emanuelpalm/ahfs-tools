use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// `[project]` section of the `project.toml` configuration file.
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionsProject {
    name: String,
    description: Option<String>,
    out_dir: Option<PathBuf>,
    version: String,
}

impl OptionsProject {
    /// Creates new default project with given `name`.
    pub fn new<N>(name: N) -> Self
        where N: Into<String>,
    {
        OptionsProject {
            name: name.into(),
            description: None,
            out_dir: None,
            version: "0.1.0".into(),
        }
    }

    /// Project name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Optional project description.
    #[inline]
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| &**s)
    }

    /// Directory where generated project files are to be stored.
    #[inline]
    pub fn out_dir(&self) -> Option<&Path> {
        self.out_dir.as_ref().map(|p| &**p)
    }

    /// Project version.
    #[inline]
    pub fn version(&self) -> &str {
        &self.version
    }
}