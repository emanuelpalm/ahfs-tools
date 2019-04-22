use ahfs;
use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    NewArgCountNot1,
    ListArgCountNot0,
    StatusArgCountNot0,
}

impl Error {
    pub fn description(&self) -> &str {
        match *self {
            Error::NewArgCountNot1 => "`new` requires one <path> argument",
            Error::ListArgCountNot0 => "`list` takes no arguments",
            Error::StatusArgCountNot0 => "`status` takes no arguments",
        }
    }
}

impl ahfs::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::NewArgCountNot1 => "R101",
            Error::ListArgCountNot0 => "R102",
            Error::StatusArgCountNot0 => "R103",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}