use std::cell::Cell;
use std::error;
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use super::{Error, Result};

pub struct Flag {
    pub short: Option<&'static str>,
    pub long: &'static str,
    pub description: &'static str,
    pub out: FlagOut,
}

pub struct FlagCell<T>(Rc<Cell<Option<T>>>);

impl<T: FromStr> FlagCell<T> {
    #[inline]
    pub fn new() -> Self {
        FlagCell(Rc::new(Cell::new(None)))
    }

    #[inline]
    pub fn take(&self) -> Option<T> {
        self.0.take()
    }
}

pub struct FlagOut {
    name: Option<&'static str>,
    out: Box<Fn(&str) -> result::Result<(), Box<error::Error>>>,
}

impl FlagOut {
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

    pub fn write<S: AsRef<str>>(&self, s: S) -> Result<()> {
        let s = s.as_ref();
        (self.out)(s).map_err(|err| Error::FlagValueInvalid {
            flag: s.into(),
            cause: err,
        })
    }
}

/*pub enum FlagOut<'a> {
    Bool(&'a FlagCell<bool>),
    I32(&'a FlagCell<i32>, &'a str),
    String(&'a FlagCell<String>, &'a str),
}

impl<'a> FlagOut<'a> {
    pub fn write<S: AsRef<str>>(&self, s: S) -> Result<'a, ()> {
        let s = s.as_ref();
        match self {
            &FlagOut::Bool(ref cell) => cell.set(match s.len() {
                0 => true,
                _ => s.parse().map_err(|_| Error::FlagInvalidBool {
                    flag: s.into(),
                })?,
            }),
            &FlagOut::I32(ref cell, _) => cell.set(s.parse()
                .map_err(|_| Error::FlagInvalidI32 {
                    flag: s.into(),
                })?),
            &FlagOut::String(ref cell, _) => cell.set(s.into()),
        };
        Ok(())
    }
}*/