use super::{Error, Lexeme, Predicate};
use super::parser;

pub struct Triple<'a> {
    subject: Lexeme<'a, ()>,
    predicate: Predicate,
    object: Lexeme<'a, ()>,
    description: Lexeme<'a, ()>,
}
