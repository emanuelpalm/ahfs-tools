use std::error;
use std::fmt;

/// A command line argument parsing error.
#[derive(Debug)]
pub enum Error {
    /// Encountered argument matches no known [`Rule`](struct.Rule.html).
    ArgUnknown(String),

    /// Encountered argument matches no known [`Flag`](struct.Flag.html).
    FlagUnknown(String),

    /// [`Flag`](struct.Flag.html) not expected.
    FlagUnexpected(String),

    /// [`Flag`](struct.Flag.html) value could not be parsed.
    FlagValueInvalid {
        flag: String,
        cause: Box<error::Error>,
    },
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ArgUnknown(_) => "Unknown argument encountered",
            &Error::FlagUnknown(_) => "Unknown flag encountered",
            &Error::FlagUnexpected(_) => "Flag not expected",
            &Error::FlagValueInvalid { .. } => "Flag value invalid"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::ArgUnknown(ref arg) => {
                write!(f, "Unknown argument `{}`", arg)
            },
            &Error::FlagUnknown(ref flag) => {
                write!(f, "Unknown flag `{}`", flag)
            },
            &Error::FlagUnexpected(ref flag) => {
                write!(f, "Unexpected flag `{}`", flag)
            },
            &Error::FlagValueInvalid { ref flag, ref cause } => {
                write!(f, "Invalid flag value `{}`, reason: {}", flag, cause)
            }
        }
    }
}