use std::error;

#[derive(Debug)]
pub enum Error {
    ArgUnknown(String),
    FlagUnknown(String),
    FlagUnexpected(String),
    FlagValueInvalid {
        flag: String,
        cause: Box<error::Error>,
    },
}