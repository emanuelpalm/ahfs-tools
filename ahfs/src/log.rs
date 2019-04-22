use std::fmt;

/// Logs message about occurrence of possible interest.
pub fn anomaly(message: &fmt::Display) {
    println!(concat!(ahfs_macro::color!(y: ">"), " {}"), message);
}

/// Logs message about intended action that completed successfully.
pub fn completion(message: &fmt::Display) {
    println!(concat!(ahfs_macro::color!(g: ">"), " {}"), message);
}

/// Logs message about intended action that failed to complete.
pub fn failure(message: &crate::Error) {
    println!(concat!(ahfs_macro::color!(r: "> [Error {}]"), " {}"), message.code(), message);
}

/// Logs suggestion to application user.
pub fn suggestion(message: &fmt::Display) {
    println!(concat!(ahfs_macro::color!(b: ">"), " {}"), message);
}