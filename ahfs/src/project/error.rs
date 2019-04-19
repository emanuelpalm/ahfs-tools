use source::Excerpt;
use std::error;
use std::fmt;

/// A project-related error.
#[derive(Debug)]
pub enum Error {
    /// Project AHFS version incompatible with current AHFS version.
    AhfsVersionIncompatible {
        version: String,
    },
    /// Project AHFS version indicator is invalid.
    AhfsVersionInvalid {
        version: String,
    },
    /// Project contains no AHFS version indicator.
    AhfsVersionMissing,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AhfsVersionIncompatible { .. } => "Incompatible AHFS version",
            Error::AhfsVersionInvalid { .. } => "Invalid AHFS version",
            Error::AhfsVersionMissing => "No AHFS version",
        }
    }
}

impl ::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::AhfsVersionIncompatible { .. } => "R001",
            Error::AhfsVersionInvalid { .. } => "R002",
            Error::AhfsVersionMissing => "R003",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))?;
        match *self {
            Error::AhfsVersionIncompatible { ref version } |
            Error::AhfsVersionInvalid { ref version } => {
                write!(f, "\n{}", version)?;
            }
            _ => {}
        }
        Ok(())
    }
}