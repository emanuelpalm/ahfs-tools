use ahfs;
use std::error;
use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    NewArgCountNot1,
    GraphArgCountNot0,
    ListArgCountNot0,
    StatusArgCountNot0,
}

impl ahfs::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::NewArgCountNot1 => "R101",
            Error::GraphArgCountNot0 => "R102",
            Error::ListArgCountNot0 => "R103",
            Error::StatusArgCountNot0 => "R104",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NewArgCountNot1 => "`new` requires one <path> argument",
            Error::GraphArgCountNot0 => "`graph` takes no arguments",
            Error::ListArgCountNot0 => "`list` takes no arguments",
            Error::StatusArgCountNot0 => "`status` takes no arguments",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(::std::error::Error::description(self))
    }
}