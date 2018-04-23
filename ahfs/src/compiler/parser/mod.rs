//! Parsing utilities.

mod state;
mod triple;

pub use self::triple::Triple;

use self::state::State;
use super::Compile;
use super::lexer::{Lexeme, LexemeKind};
use super::source::{Error, Range, Region, Result, Source, Text};

const TRIPLE_END: &'static [LexemeKind] = &[
    LexemeKind::Semicolon,
    LexemeKind::Description
];
const WORD: &'static [LexemeKind] = &[
    LexemeKind::Word
];

/// Source lexeme parser.
///
/// Transforms an array of source code lexemes into an array of
/// [`Triple`s](struct.Triple.html).
pub struct Parser;

impl<'a> Compile<'a, Box<[Lexeme<'a>]>, Box<[Triple<'a>]>> for Parser {
    fn compile(source: &'a Source<'a, Box<[Lexeme<'a>]>>)
               -> Result<'a, Source<'a, Box<[Triple<'a>]>>> {
        source.apply(|source| {
            parse(source).map(|triples| triples.into_boxed_slice())
        })
    }
}

fn parse<'a>(source: &'a Source<'a, Box<[Lexeme<'a>]>>) -> Result<'a, Vec<Triple<'a>>> {
    let mut state = State::new(source);
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
    return Ok(triples);
}

#[cfg(test)]
mod tests {
    use super::super::source::Text;
    use super::super::lexer::Lexer;
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
        let source = Lexer::compile(&source).unwrap();
        let source = Parser::compile(&source).unwrap();
        let lexeme = |kind, range| {
            Lexeme::new(kind, texts[0].get_region(range).unwrap())
        };
        assert_eq!(source.tree(), &vec![
            unsafe {
                Triple::new(
                    0..1, 2..6, 7..13,
                    lexeme(LexemeKind::Semicolon, 13..14),
                )
            },
            unsafe {
                Triple::new(
                    15..16, 17..21, 22..29,
                    lexeme(LexemeKind::Description, 30..54),
                )
            },
        ].into_boxed_slice());
    }
}