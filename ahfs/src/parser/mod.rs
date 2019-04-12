//! Arrowhead specification parsing utilities.
//!
//! This module contains tools useful for parsing specification source texts.

mod error;
mod lexer;
mod name;
mod scanner;
mod state;
mod token;
mod tree;

pub use self::error::Error;
pub use self::tree::Tree;

use self::name::Name;
use self::scanner::Scanner;
use self::state::State;
use self::token::Token;
use std::result;
use ::source::Source;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

/// Parses given source code texts into boxed slice of [`Triple`s][tri].
///
/// # Syntax
///
/// A valid source code text contains only triples. A triple is three  _words_,
/// separated by whitespace, followed by an _end_ designator. The _end_
/// designator can either be a simple semi-colon `;`, or curly braces containing
/// a description of the triple. A _word_ may consist of any characters except
/// for whitespace, `;` `{` or `}`. A description _end_ designator is closed by
/// the same number of consecutive closing curly braces it was opened with,
/// meaning that the opening and closing can be adjusted to allow patterns of
/// curly braces to be used inside a description. There is no way to express
/// comments ignored by the parser.
///
/// # Example
///
/// ```ahfs
/// Orchestrator type System;
/// Orchestrator consumes ServiceDiscovery {
///     The service is consumed to allow the Orchestrator to make itself
///     findable by other services.
/// }
/// Orchestrator produces Orchestration {{
///     As this description was opened with two consecutive `{` characters,
///     it is not closed until it encounters two consecutive `}` characters.
///     Any number of `{` can be used to open a description, as long as the
///     same number of `}` are used to close it.
/// }}
/// ```
///
/// [tri]: struct.Triple.html
pub fn parse(source: &Source) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use ::source::Text;
    use super::*;

    #[test]
    fn parse() {
        let _texts = vec![
            Text::new("alpha.ahfs", concat!(
                "import \"test.ahfs\";\n",
                "\n",
                "// This comment is ignored.\n",
                "/* This one too! */\n",
                "system TestSystem {\n",
                "    /// Comment A.\n",
                "    produces TestServiceA;\n",
                "    produces TestServiceB;\n",
                "\n",
                "    /** Comment B. */\n",
                "    consumes TestServiceX;\n",
                "    consumes TestServiceY;\n",
                "}\n",
            )),
        ];
        // TODO
    }
}