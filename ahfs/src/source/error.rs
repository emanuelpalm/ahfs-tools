use std::fmt;
use super::Region;

/// A source-related compiler error.
#[derive(Debug)]
pub struct Error<'a> {
    code: &'static str,
    text: Box<str>,
    region: Option<Region<'a>>,
}

impl<'a> Error<'a> {
    /// Creates new `Error` from given `code`, `text` and optional
    /// [`region`](struct.Region.html).
    pub fn new<S, R>(code: &'static str, text: S, region: R) -> Self
        where S: Into<Box<str>>,
              R: Into<Option<Region<'a>>>,
    {
        Error { code, text: text.into(), region: region.into() }
    }

    /// Returns machine-readable error identifier.
    #[inline]
    pub fn code(&self) -> &str {
        self.code
    }

    /// Returns human-readable error description.
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Region within a source text being subject of error, if any.
    #[inline]
    pub fn region(&self) -> Option<&Region<'a>> {
        self.region.as_ref()
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(str_color!(red: "error["), "{}", str_color!(red: "]"),
               ": {}"), self.code(), self.text())?;
        if let Some(ref region) = self.region {
            write!(f, "\n{}", region)?;
        }
        Ok(())
    }
}