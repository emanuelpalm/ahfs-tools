use std::fmt;

/// Describes a project application error.
#[derive(Debug)]
pub enum Error {
    DocArgCountNot0,
    ListArgCountNot0,
    NewArgCountNot1,
    StatusArgCountNot0,
}

impl arspec::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::DocArgCountNot0 => "R201",
            Error::ListArgCountNot0 => "R101",
            Error::NewArgCountNot1 => "R401",
            Error::StatusArgCountNot0 => "RC01",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Error::DocArgCountNot0 => "`doc` takes not arguments",
            Error::ListArgCountNot0 => "`list` takes no arguments",
            Error::NewArgCountNot1 => "`new` requires <path> argument",
            Error::StatusArgCountNot0 => "`status` takes no arguments",
        })
    }
}