use super::otf;

/// Font error type.
#[derive(Debug)]
pub enum Error {
    OTF(otf::Error),
}

impl From<otf::Error> for Error {
    #[inline]
    fn from(error: otf::Error) -> Self {
        Error::OTF(error)
    }
}