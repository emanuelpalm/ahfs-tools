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
    fn analyze(scanner: Scanner<'a>) -> Vec<Token<'a, Class>> {
        let mut tokens = Vec::new();
        inner(scanner, &mut tokens);
        return tokens;

        fn inner<'a>(mut scanner: Scanner<'a>, out: &mut Vec<Token<'a, Class>>) -> Option<()> {
            let mut ch;
            loop {
                // Pair name.
                {
                    loop {
                        ch = scanner.next()?;
                        if ch.is_whitespace() {
                            scanner.discard();
                        } else {
                            break;
                        }
                    }
                    loop {
                        if ch == ':' {
                            break;
                        }
                        ch = scanner.next()?;
                    }
                    let class = match scanner.review() {
                        "ProjectDescription:" => Class::ProjectDescription,
                        "ProjectName:" => Class::ProjectName,
                        "ProjectVersion:" => Class::ProjectVersion,
                        _ => {
                            out.push(scanner.collect(Class::InvalidName));
                            return Some(());
                        }
                    };
                    out.push(scanner.collect(class));
                }

                // Pair value.
                {
                    loop {
                        ch = scanner.next()?;
                        if ch.is_whitespace() {
                            scanner.discard();
                        } else {
                            break;
                        }
                    }
                    match ch {
                        '"' => {
                            scanner.discard();
                            loop {
                                match scanner.next()? {
                                    '"' => {
                                        scanner.unwind();
                                        out.push(scanner.collect(Class::String));
                                        scanner.next()?;
                                        break;
                                    }
                                    '\\' => { scanner.next()?; }
                                    _ => {}
                                }
                            }

                        }
                        _ => {
                            out.push(scanner.collect(Class::InvalidValue));
                            return Some(());
                        }
                    }
                }
            }
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
            let token = matcher.one(Class::String)?;
            *target = Some(token.span);
        }

        Ok(Configuration {
            name: name.map_or_else(
                || "New Project".to_string(),
                |s| s.as_str().into(),
            ),
            description: description.map(
                |s| s.as_str().into(),
            ),
            version: version.map_or_else(
                || "0.1.0".to_string(),
                |s| s.as_str().into(),
            ),
        })
    }
}

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// [span]: ../../../arspec_parser/struct.Span.html
/// [token]: ../../../arspec_parser/struct.Token.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
    InvalidName,
    InvalidValue,
    ProjectDescription,
    ProjectName,
    ProjectVersion,
    String,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Class::InvalidName => "InvalidName",
            Class::InvalidValue => "InvalidValue",
            Class::ProjectDescription => "ProjectDescription",
            Class::ProjectName => "ProjectName",
            Class::ProjectVersion => "ProjectVersion",
            Class::String => "String",
        })
    }
}