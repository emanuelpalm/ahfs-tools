use super::{Name, State, Triple};
use super::lexer;
use ::source::{Result, Source};

const TRIPLE_END: &'static [Name] = &[
    Name::Semicolon,
    Name::Description
];
const WORD: &'static [Name] = &[
    Name::Word
];

/// A `Graph` of [`Triples`](struct.Triple.html).
pub struct Graph<'a> {
    triples: Box<[Triple<'a>]>,
}

impl<'a> Graph<'a> {
    /// Creates new `Graph`from given [`triples`](struct.Triple.html).
    #[inline]
    pub fn new<T>(triples: T) -> Self
        where T: Into<Box<[Triple<'a>]>>
    {
        Graph { triples: triples.into() }
    }

    /// Parses given source code texts into boxed slice of [`Triple`s][tri].
    ///
    /// # Syntax
    ///
    /// A valid source code text contains only triples. A triple is three
    /// _words_, separated by whitespace, followed by an _end_ designator. The
    /// _end_ designator can either be a simple semi-colon `;`, or curly braces
    /// containing a description of the triple. A _word_ may consist of any
    /// characters except for whitespace, `;` `{` or `}`. A description _end_
    /// designator is closed by the same number of consecutive closing curly
    /// braces it was opened with, meaning that the opening and closing can be
    /// adjusted to allow patterns of curly braces to be used inside a
    /// description. There is no way to express comments ignored by the parser.
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
    pub fn parse(source: &'a Source) -> Result<'a, Graph<'a>> {
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
        Ok(Graph::new(triples))
    }

    /// `Graph` [`Triples`](struct.Triple.html).
    #[inline]
    pub fn triples(&self) -> &[Triple<'a>] {
        &self.triples
    }
}

#[cfg(test)]
mod tests {
    use ::source::Text;
    use super::super::Token;
    use super::*;

    #[test]
    fn parse() {
        let texts = vec![
            Text::new("alpha.ahs", concat!(
                "A type System;\n",
                "B type Service { Emojis 😜🤖💩! }",
            )),
        ];
        let source = Source::new(texts);
        let graph = super::Graph::parse(&source).unwrap();
        let token = |kind, range| {
            Token::new(kind, source.texts()[0].get_region(range).unwrap())
        };
        assert_eq!(graph.triples(), &[
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