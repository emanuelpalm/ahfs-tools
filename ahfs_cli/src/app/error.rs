use ahfs;
use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    NewArgCountNot2,
    ListArgCountNot0,
    StatusArgCountNot0,
}

impl ahfs::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::NewArgCountNot2 => "R101",
            Error::ListArgCountNot0 => "R102",
            Error::StatusArgCountNot0 => "R103",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Error::NewArgCountNot2 => "`new` requires <name> and <path> arguments",
            Error::ListArgCountNot0 => "`list` takes no arguments",
            Error::StatusArgCountNot0 => "`status` takes no arguments",
        })
    }
}