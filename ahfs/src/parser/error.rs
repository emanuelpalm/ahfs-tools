use parser::Name;
use source::Excerpt;
use std::error;
use std::fmt;

/// A parser error.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Error {
    /// There is nothing to parse.
    NoSource,

    /// A parsed source [`Text`](../source/struct.Text.html) ended unexpectedly.
    UnexpectedSourceEnd {
        excerpt: Excerpt,
        expected: Vec<Name>,
    },

    /// An unexpected [`Token`](struct.Token.html) was read while parsing.
    UnexpectedToken {
        name: Name,
        excerpt: Excerpt,
        expected: Vec<Name>,
    },
}

impl Error {
    pub fn push_expected(&mut self, names: &[Name]) {
        match *self {
            Error::NoSource => {},
            Error::UnexpectedSourceEnd { ref mut expected, .. } |
            Error::UnexpectedToken { ref mut expected, .. } => {
                for name in names {
                    expected.push(*name);
                }
            }
        };
    }
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

            let (first, rest) = expected.split_first().unwrap();
            write!(f, "`{}`", first)?;

            let (last, rest) = rest.split_last().unwrap();
            for item in rest {
                write!(f, ", `{}`", item)?;
            }
            write!(f, " or `{}`", last)?
        }
    };
    write!(f, ".\n{}", excerpt)
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

impl ::Error for Error {
    fn error_code(&self) -> &'static str {
        match *self {
            Error::NoSource => "P001",
            Error::UnexpectedSourceEnd { .. } => "P002",
            Error::UnexpectedToken { .. } => "P003",
        }
    }
}