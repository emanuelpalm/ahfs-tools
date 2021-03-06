use crate::project;
use crate::spec;
use std::fmt;
use std::io;
use std::result;

/// A generic ARSPEC result.
pub type Result<T = ()> = result::Result<T, Box<dyn Error>>;

/// Error trait to be implemented by most ARSPEC error types.
pub trait Error: fmt::Debug + fmt::Display {
    /// Machine-readable error code.
    fn code(&self) -> &'static str;

    /// Tries to cast error into an I/O `Error`.
    fn as_io_error(&self) -> Option<&io::Error> {
        None
    }
}

impl Error for arspec_parser::Error<project::parser::Class> {
    fn code(&self) -> &'static str {
        match self.actual {
            None => "PP01",
            Some(_) => "PP02",
        }
    }
}

impl Error for arspec_parser::Error<spec::parser::Class> {
    fn code(&self) -> &'static str {
        match self.actual {
            None => "PS01",
            Some(_) => "PS02",
        }
    }
}

impl<'a> Error for spec::VerificationError {
    fn code(&self) -> &'static str {
        match self {
            &spec::VerificationError::EnumNameDuplicate { .. } => "VE01",
            &spec::VerificationError::EnumVariantDuplicate { .. } => "VE02",
            &spec::VerificationError::InterfaceNotImplemented { .. } => "VE03",
            &spec::VerificationError::NoSuchInterfaceToImplement { .. } => "VE04",
            &spec::VerificationError::NoSuchServiceToImplement { .. } => "VE05",
            &spec::VerificationError::UnknownServiceEncoding { .. } => "VE06",
            &spec::VerificationError::UnknownServiceProtocol { .. } => "VE07",
            &spec::VerificationError::PrimitiveNameDuplicate { .. } => "VE08",
            &spec::VerificationError::RecordNameDuplicate { .. } => "VE09",
            &spec::VerificationError::ServiceNameDuplicate { .. } => "VE10",
            &spec::VerificationError::SystemNameDuplicate { .. } => "VE11",
        }
    }
}

impl Error for fmt::Error {
    #[inline]
    fn code(&self) -> &'static str {
        "FMT1"
    }
}

impl Error for io::Error {
    fn code(&self) -> &'static str {
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

    fn as_io_error(&self) -> Option<&io::Error> {
        Some(self)
    }
}

impl<E: Error + 'static> From<E> for Box<dyn Error> {
    #[inline]
    fn from(err: E) -> Self {
        Box::new(err) as Box<dyn Error>
    }
}
