use arspec_macro::color;
use std::fmt;

/// Logs message about occurrence of possible interest.
pub fn anomaly(message: &dyn fmt::Display) {
    println!(concat!(color!(y: ">"), " {}"), message);
}

/// Logs message about intended action that completed successfully.
pub fn completion(message: &dyn fmt::Display) {
    println!(concat!(color!(g: ">"), " {}"), message);
}

/// Logs message about intended action that failed to complete.
pub fn failure(message: &dyn crate::Error) {
    println!(concat!(color!(r: "> [Error {}]"), " {}"), message.code(), message);
}

/// Logs suggestion to application user.
pub fn suggestion(message: &dyn fmt::Display) {
    println!(concat!(color!(b: ">"), " {}"), message);
}