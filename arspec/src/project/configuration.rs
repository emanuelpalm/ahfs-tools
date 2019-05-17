use arspec_parser::Text;
use crate::error::Result;
use std::fs;
use std::io::Write;
use std::path::Path;
use super::parser;

/// Arrowhead Framework project configuration.
#[derive(Debug)]
pub struct Configuration {
    /// Project name.
    pub name: String,

    /// Optional project description.
    pub description: Option<String>,

    /// Project version.
    pub version: String,
}

impl Configuration {
    /// Attempts to create `Configuration` file at given `path`, which is also
    /// returned.
    pub fn new<N>(name: N) -> Self
        where N: Into<String>,
    {
        Configuration {
            name: name.into(),
            description: None,
            version: "0.1.0".into(),
        }
    }

    /// Attempts to read `Configuration` from file at given `path`.
    #[inline]
    pub fn read_at<P>(path: P) -> Result<Configuration>
        where P: AsRef<Path>,
    {
        let text = Text::read_at(path)?;
        let conf = parser::parse(&text)?;
        Ok(conf)
    }

    /// Writes `Configuration` to file at given `path`.
    pub fn write_to<P>(&self, path: P) -> Result
        where P: AsRef<Path>,
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.as_ref())?;

        file.write_all(format!(
            concat!(
                "ProjectName: \"{}\"\n",
                "ProjectVersion: \"{}\"",
            ),
            self.name.replace('\"', "\\\""),
            self.version.replace('\"', "\\\""),
        ).as_bytes())?;

        if let Some(ref description) = self.description {
            file.write_all(format!(
                "ProjectDescription: \"{}\"\n",
                description.replace('\"', "\\\""),
            ).as_bytes())?;
        }

        Ok(())
    }
}
