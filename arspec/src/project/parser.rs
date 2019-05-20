use arspec_parser::{Error, Matcher, Parser, Scanner, Span, Text, Token};
use std::fmt;
use super::Configuration;

/// Attempt to create [`Configuration`][cnf] from given source [`text`][txt].
///
/// [cnf]: ../struct.Configuration.html
/// [txt]: ../../../arspec_parser/struct.Text.html
#[inline]
pub fn parse(text: &Text) -> Result<Configuration, Error<Class>> {
    ConfigurationParser::parse(text)
}

struct ConfigurationParser;

impl<'a> Parser<'a> for ConfigurationParser {
    type Class = Class;
    type Output = Configuration;

    #[inline]
    fn analyze(mut scanner: Scanner<'a>) -> Vec<Token<'a, Class>> {
        let mut tokens = Vec::new();
        scan_all(&mut scanner, &mut tokens);
        return tokens;

        fn scan_all<'a>(scanner: &mut Scanner<'a>, out: &mut Vec<Token<'a, Class>>) -> Option<()> {
            let mut ch;
            loop {
                ch = scanner.next()?;

                if ch.is_whitespace() {
                    scanner.discard();
                    continue;
                }

                let class = match ch {
                    ':' => Class::Colon,
                    '"' => scan_string(scanner)?,
                    'P' => scan_symbol(scanner)?,
                    _ => Class::UnknownSymbol,
                };

                out.push(scanner.collect(class));
            }
        }

        fn scan_string(scanner: &mut Scanner) -> Option<Class> {
            let mut class = Class::String;
            loop {
                match scanner.next()? {
                    '"' => {
                        return Some(class);
                    }
                    '\\' => match scanner.next()? {
                        '"' | '\\' | 'n' | 'r' | 't' => {}
                        _ => {
                            class = Class::InvalidStringEscape;
                        }
                    },
                    _ => {}
                }
            }
        }

        fn scan_symbol(scanner: &mut Scanner) -> Option<Class> {
            loop {
                let ch = scanner.next()?;
                if !ch.is_ascii_alphabetic() {
                    scanner.unwind();
                    break;
                }
            }
            Some(match scanner.review() {
                "ProjectDescription" => Class::ProjectDescription,
                "ProjectName" => Class::ProjectName,
                "ProjectVersion" => Class::ProjectVersion,
                _ => Class::UnknownSymbol,
            })
        }
    }

    #[inline]
    fn combine(mut matcher: Matcher<'a, Class>) -> Result<Configuration, Error<Class>> {
        let mut name: Option<Span<'a>> = None;
        let mut description: Option<Span<'a>> = None;
        let mut version: Option<Span<'a>> = None;

        while !matcher.at_end() {
            let token = matcher.any(&[
                Class::ProjectDescription,
                Class::ProjectName,
                Class::ProjectVersion,
            ])?;
            let target = match token.kind {
                Class::ProjectDescription => &mut description,
                Class::ProjectName => &mut name,
                Class::ProjectVersion => &mut version,
                _ => unreachable!(),
            };
            matcher.one(Class::Colon)?;
            let token = matcher.one(Class::String)?;

            *target = Some(token.span);
        }

        return Ok(Configuration {
            name: name.map_or_else(
                || "New Project".to_string(),
                span_to_string,
            ),
            description: description.map(
                span_to_string,
            ),
            version: version.map_or_else(
                || "0.1.0".to_string(),
                span_to_string,
            ),
        });

        fn span_to_string(span: Span) -> String {
            let input = span.as_str();
            let input = &input[1..input.len() - 1];
            let mut output = String::with_capacity(input.len());

            let mut chars = input.chars();
            loop {
                let ch = match chars.next() {
                    Some(ch) => match ch {
                        '\\' => match chars.next() {
                            Some('t') => '\t',
                            Some('n') => '\n',
                            Some('r') => '\r',
                            Some(ch) => ch,
                            None => {
                                break;
                            }
                        }
                        ch => ch,
                    }
                    None => {
                        break;
                    }
                };
                output.push(ch);
            }
            output
        }
    }
}

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// [span]: ../../../arspec_parser/struct.Span.html
/// [token]: ../../../arspec_parser/struct.Token.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
    // Delimiters.
    Colon,

    // Symbols.
    ProjectDescription,
    ProjectName,
    ProjectVersion,

    // Literals.
    String,

    // Errors.
    InvalidStringEscape,
    UnknownSymbol,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Class::Colon => ":",

            Class::ProjectDescription => "ProjectDescription",
            Class::ProjectName => "ProjectName",
            Class::ProjectVersion => "ProjectVersion",

            Class::String => "String",

            Class::InvalidStringEscape => "<InvalidStringEscape>",
            Class::UnknownSymbol => "<UnknownSymbol>",
        })
    }
}