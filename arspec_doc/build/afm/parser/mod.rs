mod class;
mod lexer;
mod parser;

use arspec_parser::{Error, Matcher, Parser, Scanner, Text, Token};
use self::class::Class;
use std::fmt;
use super::FontMetrics;

/// Attempt to create [`Configuration`][cnf] from given source [`text`][txt].
///
/// [cnf]: ../struct.Configuration.html
/// [txt]: ../../../arspec_parser/struct.Text.html
#[inline]
pub fn parse(text: &Text) -> Result<FontMetrics, Error<Class>> {
    AFMParser::parse(text)
}

struct AFMParser;

impl<'a> Parser<'a> for AFMParser {
    type Class = Class;
    type Output = FontMetrics;

    #[inline]
    fn analyze(scanner: Scanner<'a>) -> Vec<Token<'a, Class>> {
        return lexer::scan(scanner);
    }

    #[inline]
    fn combine(mut matcher: Matcher<'a, Class>) -> Result<FontMetrics, Error<Class>> {
        parser::root(&mut matcher)
    }
}

