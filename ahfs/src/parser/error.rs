use ::pest;
use std::fmt;
use super::grammar::Rule;
use super::Lexeme;

#[derive(Debug)]
pub struct Error<'a> {
    inner: pest::Error<'a, Rule>,
}

impl<'a> fmt::Display for Error<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl<'a> Error<'a> {
    pub fn new<M>(lexeme: Lexeme<'a>, message: M) -> Self
        where M: Into<String>
    {
        Error {
            inner: pest::Error::CustomErrorSpan {
                message: message.into(),
                span: lexeme.into(),
            }
        }
    }
}

impl<'a> From<pest::Error<'a, Rule>> for Error<'a> {
    #[inline]
    fn from(error: pest::Error<'a, Rule>) -> Self {
        Error {
            inner: error.renamed_rules(|rule| {
                match *rule {
                    Rule::triple => "invalid triple",
                    Rule::identifier => "invalid identifier",
                    _ => "unexpected input",
                }.to_owned()
            }),
        }
    }
}