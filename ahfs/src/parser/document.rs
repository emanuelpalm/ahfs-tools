use ::pest::Parser;
use super::grammar::{Grammar, Rule};
use super::{Error, Lexeme, Triple};
use std::result;

pub struct Document<'a> {
    triples: Box<[Triple<'a>]>,
}

impl<'a> Document<'a> {
    pub fn parse(source: &'a str) -> result::Result<Document<'a>, Error<'a>> {
        Grammar::parse(Rule::document, source)
            .map(|mut pairs| Document {
                triples: pairs.next()
                    .unwrap()
                    .into_inner()
                    .fold(Vec::new(), |mut triples, pair| {
                        let mut lexemes: Vec<Lexeme<'a>> = pair.into_inner()
                            .map(|pair| pair.into_span().into())
                            .collect();

                        let object: Lexeme<'a> = lexemes.remove(2);
                        let predicate: Lexeme<'a> = lexemes.remove(1);
                        let subject: Lexeme<'a> = lexemes.remove(0);
                        triples.push(Triple::new(subject, predicate, object));

                        triples
                    })
                    .into_boxed_slice()
            })
            .map_err(|error| Error::from(error))
    }

    #[inline]
    pub fn triples(&self) -> &[Triple<'a>] {
        &self.triples
    }
}