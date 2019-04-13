use std::error;
use std::fmt;
use super::Name;
use ::source::Excerpt;

/// A parser error.
#[derive(Debug)]
pub enum Error {
    /// There is nothing to parse.
    NoSource,

    /// A parsed source [`Text`](../source/struct.Text.html) ended unexpectedly.
    UnexpectedSourceEnd {
        excerpt: Excerpt,
        expected: Box<[Name]>,
    },

    /// An unexpected [`Token`](struct.Token.html) was read while parsing.
    UnexpectedToken {
        name: Name,
        excerpt: Excerpt,
        expected: Box<[Name]>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoSource => write!(f, "No known sources"),
            Error::UnexpectedSourceEnd { ref excerpt, ref expected } => {
                write!(f, "Unexpected source end")?;
                write_unexpected(f, &expected, excerpt)
            }
            Error::UnexpectedToken { name, ref excerpt, ref expected } => {
                write!(f, "Unexpected `{}`", name)?;
                write_unexpected(f, &expected, excerpt)
            }
        }?;
        Ok(())
    }
}

fn write_unexpected(
    f: &mut fmt::Formatter,
    expected: &[Name],
    excerpt: &Excerpt,
) -> fmt::Result
{
    match expected.len() {
        0 => {},
        1 => write!(f, ", expected `{}`", expected[0])?,
        _ => {
            write!(f, ", expected one of ")?;
            let (last, rest) = expected.split_last().unwrap();
            for item in rest {
                write!(f, "`{}`, ", item)?;
            }
            write!(f, " or `{}`", last)?
        }
    };
    write!(f, "\n{}", excerpt)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoSource => "No known sources",
            Error::UnexpectedSourceEnd { .. } => "Unexpected source end",
            Error::UnexpectedToken { .. } => "Unexpected token",
        }
    }
}

impl ::ErrorCode for Error {
    fn error_code(&self) -> &'static str {
        match *self {
            Error::NoSource => "P001",
            Error::UnexpectedSourceEnd { .. } => "P002",
            Error::UnexpectedToken { .. } => "P003",
        }
    }
}