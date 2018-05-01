//! Arrowhead specification parsing utilities.
//!
//! This module contains tools useful for parsing specification source texts.

mod lexer;
mod name;
mod scanner;
mod state;
mod token;

pub use self::name::Name;
pub use self::token::Token;

use self::state::State;
use super::source::{Error, Range, Region, Result, Source, Text};
use super::Triple;

const TRIPLE_END: &'static [Name] = &[
    Name::Semicolon,
    Name::Description
];
const WORD: &'static [Name] = &[
    Name::Word
];

/// Attempts to parse given source code texts into boxed slice of
/// [`Triple`s](struct.Triple.html).
pub fn parse<'a>(source: &Source<'a>) -> Result<'a, Box<[Triple<'a>]>> {
    let tokens = lexer::analyze(source);
    let mut state = State::new(&tokens);
    let mut triples = Vec::new();
    while !state.at_end() {
        triples.push(state.apply(|state| {
            let subject = state.next_if(WORD)?;
            let predicate = state.next_if(WORD)?;
            let object = state.next_if(WORD)?;
            let end = state.next_if(TRIPLE_END)?;
            Ok(unsafe { Triple::new(subject, predicate, object, end) })
        })?);
    }
    Ok(triples.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let texts: &[Text] = &[
            Text::new("alpha.ahs", concat!(
                "A type System;\n",
                "B type Service { Emojis 😜🤖💩! }",
            )),
        ];
        let source = Source::new(texts);
        let triples = super::parse(&source).unwrap();
        let token = |kind, range| {
            Token::new(kind, texts[0].get_region(range).unwrap())
        };
        assert_eq!(triples.as_ref(), &[
            unsafe {
                Triple::new(
                    0..1, 2..6, 7..13,
                    token(Name::Semicolon, 13..14),
                )
            },
            unsafe {
                Triple::new(
                    15..16, 17..21, 22..29,
                    token(Name::Description, 30..54),
                )
            },
        ]);
    }
}