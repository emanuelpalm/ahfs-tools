use std::error;
use std::fmt;
use ::source::Excerpt;

#[derive(Debug)]
pub enum Error {
    AhfsVersionIncompatible {
        excerpt: Excerpt,
    },
    AhfsVersionInvalid {
        excerpt: Excerpt,
    },
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

impl ::ErrorCode for Error {
    fn error_code(&self) -> &'static str {
        match *self {
            Error::AhfsVersionIncompatible { .. } => "R001",
            Error::AhfsVersionInvalid { .. } => "R002",
            Error::AhfsVersionMissing => "R003",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::ErrorCode::fmt(self, f)?;
        write!(f, " {}", error::Error::description(self))?;
        match *self {
            Error::AhfsVersionIncompatible { ref excerpt } |
            Error::AhfsVersionInvalid { ref excerpt } => {
                write!(f, "\n{}", excerpt)?;
            }
            _ => {}
        }
        Ok(())
    }
}