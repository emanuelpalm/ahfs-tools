use ahfs;
use std::error;
use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    NewArgCountNot1,
    StatusArgCountNot0,
}

impl ahfs::ErrorCode for Error {
    fn error_code(&self) -> &'static str {
        match *self {
            Error::NewArgCountNot1 => "R101",
            Error::StatusArgCountNot0 => "R102",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NewArgCountNot1 => "`new` requires one <path> argument",
            Error::StatusArgCountNot0 => "`status` takes no arguments"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(::std::error::Error::description(self))
    }
}