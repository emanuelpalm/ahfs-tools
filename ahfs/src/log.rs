use std::fmt;

/// Logs message about occurrence of possible interest.
pub fn anomaly(message: &fmt::Display) {
    println!(concat!(str_color!(yellow: ">"), " {}"), message);
}

/// Logs message about intended action that completed successfully.
pub fn completion(message: &fmt::Display) {
    println!(concat!(str_color!(green: ">"), " {}"), message);
}

/// Logs message about intended action that failed to complete.
pub fn failure(message: &::ErrorCode) {
    println!(concat!(str_color!(red: "> [Error {}]"), " {}"), message.error_code(), message);
}

/// Logs suggestion to application user.
pub fn suggestion(message: &fmt::Display) {
    println!(concat!(str_color!(blue: ">"), " {}"), message);
}