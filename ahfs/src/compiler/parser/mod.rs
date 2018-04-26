//! Parsing utilities.

mod state;
mod triple;

pub use self::triple::Triple;

use self::state::State;
use super::lexer::{Name, Token, TokenTree};
use super::source::{Error, Range, Region, Result, Source, Text};
use super::Tree;

const TRIPLE_END: &'static [Name] = &[
    Name::Semicolon,
    Name::Description
];
const WORD: &'static [Name] = &[
    Name::Word
];

/// A [`Tree`](../struct.Tree.html) of [`Triple`s](struct.Triple.html).
pub type ParseTree<'a> = Tree<'a, [Triple<'a>]>;

/// [`Token`](../lexer/struct.Token.html) parser.
///
/// Transforms an array of source code Tokens into an array of
/// [`Triple`s](struct.Triple.html).
pub struct Parser;

impl Parser {
    pub fn parse<'a>(tokens: TokenTree<'a>) -> Result<'a, ParseTree<'a>> {
        let mut triples = Vec::new();
        {
            let mut state = State::new(&tokens);
            while !state.at_end() {
                triples.push(state.apply(|state| {
                    let subject = state.next_if(WORD)?;
                    let predicate = state.next_if(WORD)?;
                    let object = state.next_if(WORD)?;
                    let end = state.next_if(TRIPLE_END)?;
                    Ok(unsafe { Triple::new(subject, predicate, object, end) })
                })?);
            }
        }
        Ok(ParseTree::new(tokens, triples))
    }
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
        let tree = Parser::parse(Lexer::analyze(source)).unwrap();
        let triples = tree.root();
        let token = |kind, range| {
            Token::new(kind, texts[0].get_region(range).unwrap())
        };
        assert_eq!(&triples, &[
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