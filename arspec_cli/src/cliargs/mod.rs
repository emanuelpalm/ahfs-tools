//! Command Line Interface (CLI) argument utilities.
//!
//! This module contains tools useful for managing command line arguments.

mod parser;
mod rule;
mod error;
mod flag;

pub use self::parser::Parser;
pub use self::rule::Rule;
pub use self::error::Error;
pub use self::flag::{Flag, FlagCell, FlagOut};

use std::result;

/// The result of parsing command line arguments.
pub type Result<T = ()> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app() {
        let verbose = FlagCell::new();
        let parser = Parser {
            description: "A CLI application.",
            rules: &[
                Rule {
                    name: "help",
                    name_details: "",
                    description: "Lists all available commands.",
                    flags: &[
                        Flag {
                            short: Some("v"),
                            long: "verbose",
                            description: "",
                            out: FlagOut::new_bool(&verbose),
                        }
                    ],
                    callback: &|_args| Ok(()),
                }
            ],
        };
        assert_eq!(None, verbose.take());
        parser.parse(&[
            "help".to_string(),
            "-v".to_string(),
        ]).unwrap();
        assert_eq!(Some(true), verbose.take());
    }
}