use std::error;
use std::fmt;

/// A command line argument parsing error.
#[derive(Debug)]
pub enum Error {
    /// Encountered argument matches no known [`Rule`](struct.Rule.html).
    ArgUnknown(String),

    /// Encountered argument matches no known [`Flag`](struct.Flag.html).
    FlagUnknown(String),

    /// [`Flag`](struct.Flag.html) value could not be parsed.
    FlagValueInvalid {
        flag: String,
        cause: Box<error::Error>,
    },

    /// A command line rule failed to complete.
    RuleFailed(Box<arspec::Error>),
}

impl arspec::Error for Error {
    fn code(&self) -> &'static str {
        match *self {
            Error::ArgUnknown(_) => "C001",
            Error::FlagUnknown(_) => "C002",
            Error::FlagValueInvalid { .. } => "C003",
            Error::RuleFailed(ref err) => err.code(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ArgUnknown(ref arg) => {
                write!(f, "Unknown argument `{}`", arg)
            }
            Error::FlagUnknown(ref flag) => {
                write!(f, "Unknown flag `{}`", flag)
            }
            Error::FlagValueInvalid { ref flag, ref cause } => {
                write!(f, "Invalid flag value `{}`, reason:\n{}", flag, cause)
            }
            Error::RuleFailed(ref err) => {
                fmt::Display::fmt(err, f)
            }
        }
    }
}