use std::cell::Cell;
use std::error;
use std::fmt;
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use super::{Error, Result};

/// Describes a command line flag.
pub struct Flag {
    /// The short form of the flag, if any.
    pub short: Option<&'static str>,

    /// The long form, or common name, of the flag.
    pub long: &'static str,

    /// Description of flag.
    pub description: &'static str,

    /// Reference to [`FlagCell`](struct.FlagCell.html).
    pub out: FlagOut,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(short) = self.short {
            write!(f, "-{:2} ", short)?;
        } else {
            f.write_str("    ")?;
        }
        let mut len = self.long.len();
        write!(f, "--{}", self.long)?;
        if let Some(value_name) = self.out.name {
            write!(f, "={}", value_name)?;
            len += 1 + value_name.len();
        }
        write!(f, "{:offset$} {}",
               "", self.description, offset = 19usize.saturating_sub(len))
    }
}

/// Holds the value of a parsed command line flag.
pub struct FlagCell<T>(Rc<Cell<Option<T>>>);

impl<T: FromStr> FlagCell<T> {
    /// Creates new `FlagCell`.
    #[inline]
    pub fn new() -> Self {
        FlagCell(Rc::new(Cell::new(None)))
    }

    /// Sets cell value to `v`, replacing any existing value.
    #[inline]
    pub fn set(&self, v: T) {
        self.0.set(Some(v))
    }

    /// Takes the value of the cell, leaving `Default::default()` in its place.
    #[inline]
    pub fn take(&self) -> Option<T> {
        self.0.take()
    }

    /// Takes the value of the cell, or provides `def` if not set.
    #[inline]
    pub fn take_or(&self, def: T) -> T {
        self.take().unwrap_or(def)
    }
}

/// A reference to a [`FlagCell`](struct.FlagCell.html).
pub struct FlagOut {
    name: Option<&'static str>,
    out: Box<Fn(&str) -> result::Result<(), Box<error::Error>>>,
}

impl FlagOut {
    /// Creates new flag cell, holding a boolean value.
    pub fn new(cell: &FlagCell<bool>) -> Self {
        let cell: Rc<_> = cell.0.clone();
        FlagOut {
            name: None,
            out: Box::new(move |s| {
                cell.set(Some(match s.len() {
                    0 => true,
                    _ => s.parse().map_err(|err| Box::new(err))?,
                }));
                Ok(())
            }),
        }
    }

    /// Creates new flag cell, holding an arbitrary value described by `name`.
    pub fn with_value<T, E>(name: &'static str, cell: &FlagCell<T>) -> Self
        where T: FromStr<Err=E> + 'static,
              E: error::Error + 'static,
    {
        let cell: Rc<_> = cell.0.clone();
        FlagOut {
            name: Some(name),
            out: Box::new(move |s| {
                cell.set(Some(T::from_str(s).map_err(|err| Box::new(err))?));
                Ok(())
            }),
        }
    }

    /// Name of flag out value.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name
    }

    /// Writes given flag value `s` to cell.
    #[doc(hidden)]
    pub fn write<S: AsRef<str>>(&self, s: S) -> Result<()> {
        let s = s.as_ref();
        (self.out)(s).map_err(|err| Error::FlagValueInvalid {
            flag: s.into(),
            cause: err,
        })
    }
}