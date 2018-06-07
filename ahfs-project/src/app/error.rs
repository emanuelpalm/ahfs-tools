use ahfs;
use std::error;
use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    NewArgMissing,
}

impl ahfs::ErrorCode for Error {
    fn error_code(&self) -> &'static str {
        match *self {
            Error::NewArgMissing => "R101",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NewArgMissing => "`new` requires exactly one path argument.",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ahfs::ErrorCode::fmt(self, f)?;
        f.write_str(::std::error::Error::description(self))
    }
}