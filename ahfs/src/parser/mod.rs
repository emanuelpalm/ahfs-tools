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
use self::state::{RuleState, State};
use self::token::Token;
use source::Source;
use std::result;

/// The `Result` of parsing.
pub type Result<T> = result::Result<T, Error>;

/// Parses given source code texts.
pub fn parse(source: &Source) -> Result<Tree> {
    let tokens = lexer::analyze(source);
    let mut state = State::new(&tokens);

    return state.apply(|mut state| {
        let mut imports = Vec::new();
        let mut systems = Vec::new();

        while !state.at_end() {
            let token = state.any(&[Name::Import, Name::System])?;
            match *token.name() {
                Name::Import => parse_import(&mut state, &mut imports)?,
                Name::System => parse_system(&mut state, &mut systems)?,
                _ => unreachable!(),
            }
        }

        Ok(Tree {
            imports: imports.into(),
            systems: systems.into(),
        })
    });

    fn parse_import<'a, 'b: 'a>(state: &mut RuleState<'a, 'b>, out: &mut Vec<tree::Import<'b>>) -> Result<()> {
        let head = state.all(&[Name::String, Name::Semicolon])?;
        out.push(tree::Import {
            name: head[0].region().clone(),
        });
        Ok(())
    }

    fn parse_system<'a, 'b: 'a>(state: &mut RuleState<'a, 'b>, out: &mut Vec<tree::System<'b>>) -> Result<()> {
        let mut consumes = Vec::new();
        let mut produces = Vec::new();

        let head = state.all(&[Name::Identifier, Name::BraceLeft])?;
        loop {
            let next = state.any(&[
                Name::Consumes,
                Name::Produces,
                Name::Comment,
                Name::BraceRight,
            ])?;
            match *next.name() {
                Name::Consumes => {
                    let head = state.all(&[Name::Identifier, Name::Semicolon])?;
                    consumes.push(head[0].region().clone());
                }
                Name::Produces => {
                    let head = state.all(&[Name::Identifier, Name::Semicolon])?;
                    produces.push(head[0].region().clone());
                }
                Name::Comment => {}
                Name::BraceRight => { break; }
                _ => unreachable!(),
            }
        }

        out.push(tree::System {
            name: head[0].region().clone(),
            consumes: consumes.into(),
            produces: produces.into(),
            comment: None,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::source::Source;
    use crate::source::Text;

    #[test]
    fn parse() {
        let texts = vec![
            Text::new("alpha.ahfs", concat!(
                "import \"test.ahfs\";\n",
                "\n",
                "// This comment is ignored.\n",
                "/* This one too! */\n",
                "system TestSystem {\n",
                "    /// Comment A.\n",
                "    consumes TestServiceX;\n",
                "    consumes TestServiceY;\n",
                "\n",
                "    /** Comment B. */\n",
                "    produces TestServiceA;\n",
                "    produces TestServiceB;\n",
                "}\n",
            )),
        ];
        let source = Source::new(texts);
        if let Err(error) = super::parse(&source) {
            println!("{}", error);
            panic!("{:?}", error);
        }
    }
}