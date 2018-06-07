use std::error;
use std::fmt;
use std::io;
use std::result;

/// A boxed AHFS error.
pub type Error = Box<ErrorCode>;

/// A generic AHFS result.
pub type Result<T> = result::Result<T, Error>;

/// Error trait implemented by all AHFS error types.
pub trait ErrorCode: error::Error {
    /// Machine-readable error code.
    ///
    /// Error codes exist to assist machine reading of error messages. Each kind
    /// of error should, if possible, provide its own unique code.
    fn error_code(&self) -> &'static str;

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, concat!(
            str_color!( red: "error[{}]"),
            str_color!(none: ": ")),
               self.error_code())
    }
}

impl ErrorCode for io::Error {
    fn error_code(&self) -> &'static str {
        match self.kind() {
            io::ErrorKind::NotFound => "F001",
            io::ErrorKind::PermissionDenied => "F002",
            io::ErrorKind::ConnectionRefused => "F003",
            io::ErrorKind::ConnectionReset => "F004",
            io::ErrorKind::ConnectionAborted => "F005",
            io::ErrorKind::NotConnected => "F006",
            io::ErrorKind::AddrInUse => "F007",
            io::ErrorKind::AddrNotAvailable => "F008",
            io::ErrorKind::BrokenPipe => "F009",
            io::ErrorKind::AlreadyExists => "F010",
            io::ErrorKind::WouldBlock => "F011",
            io::ErrorKind::InvalidInput => "F012",
            io::ErrorKind::InvalidData => "F013",
            io::ErrorKind::TimedOut => "F014",
            io::ErrorKind::WriteZero => "F015",
            io::ErrorKind::Interrupted => "F016",
            io::ErrorKind::UnexpectedEof => "F017",
            _ => "F0XX",
        }
    }
}

impl<E: ErrorCode + 'static> From<E> for Error {
    #[inline]
    fn from(err: E) -> Self {
        Box::new(err) as Error
    }
}