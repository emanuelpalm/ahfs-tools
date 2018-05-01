use std::fmt;
use super::Region;

/// A source-related compiler error.
#[derive(Debug)]
pub struct Error<'a> {
    code: &'static str,
    description: Box<str>,
    region: Option<Region<'a>>,
}

impl<'a> Error<'a> {
    /// Creates new `Error` from given `code`, `description` and optional
    /// [`region`](struct.Region.html).
    pub fn new<D, R>(code: &'static str, description: D, region: R) -> Self
        where D: Into<Box<str>>,
              R: Into<Option<Region<'a>>>,
    {
        Error { code, description: description.into(), region: region.into() }
    }

    /// Returns machine-readable error identifier.
    #[inline]
    pub fn code(&self) -> &str {
        self.code
    }

    /// Returns description of error.
    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Region within a source text being subject of error, if any.
    #[inline]
    pub fn region(&self) -> Option<&Region<'a>> {
        self.region.as_ref()
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            str_color!( red: "error["),
            str_color!(none: "{}"),
            str_color!( red: "]"),
            str_color!(none: ": {}")),
               self.code(), self.description())?;
        if let Some(ref region) = self.region {
            write!(f, "\n{}", region)?;
        }
        Ok(())
    }
}