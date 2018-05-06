use std::error;
use std::io;

pub trait ErrorCode: error::Error {
    fn error_code(&self) -> &'static str;
}

impl ErrorCode for io::Error {
    fn error_code(&self) -> &'static str {
        match self.kind() {
            io::ErrorKind::NotFound => "F000",
            io::ErrorKind::PermissionDenied => "F001",
            io::ErrorKind::Interrupted => "F002",
            _ => "F00X",
        }
    }
}