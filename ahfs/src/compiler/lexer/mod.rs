//! Lexical analysis utilities.

mod lexeme;
mod lexeme_kind;
mod reader;

pub use self::lexeme::Lexeme;
pub use self::lexeme_kind::LexemeKind;

use self::reader::Reader;
use super::Compile;
use super::source::{Range, Region, Result, Source, Text};

macro_rules! next_or_break {
    ($source:expr) => (match $source.next() { Some(c) => c, None => break });
}
macro_rules! peek_or_break {
    ($source:expr) => (match $source.peek() { Some(c) => c, None => break });
}

/// Lexical analyzer.
///
/// Transforms a set of source texts into an array of
/// [`Lexeme`s](struct.Lexeme.html).
pub struct Lexer;

impl<'a> Compile<'a, (), Box<[Lexeme<'a>]>> for Lexer {
    fn compile(source: &'a Source<'a, ()>)
        -> Result<'a, Source<'a, Box<[Lexeme<'a>]>>> {
        source.apply(|source| {
            let mut lexemes = Vec::new();
            for text in source.texts() {
                analyze(text, &mut lexemes);
            }
            Ok(lexemes.into_boxed_slice())
        })
    }
}

fn analyze<'a>(text: &'a Text<'a>, out: &mut Vec<Lexeme<'a>>) {
    let mut reader = Reader::new(text);
    loop {
        let mut c = next_or_break!(reader);
        let kind = match c {

            // Whitespace.
            b'\0'...b' ' | 0x7f => {
                reader.discard();
                continue;
            }

            // Delimiter.
            b';' => LexemeKind::Semicolon,
            b'}' => LexemeKind::BraceRight,

            // Description.
            b'{' => {
                let mut left_braces = 1;
                loop {
                    c = next_or_break!(reader);
                    if c != b'{' { break; }
                    left_braces += 1;
                }
                let mut right_braces = 0;
                loop {
                    if c == b'}' {
                        right_braces += 1;
                        if left_braces == right_braces { break; }
                    } else {
                        right_braces = 0;
                    }
                    c = next_or_break!(reader);
                }
                LexemeKind::Description
            }

            // Word.
            _ => {
                loop {
                    match peek_or_break!(reader) {
                        b'\0'...b' ' | b';' | b'{' | b'}' | 0x7f => {
                            break;
                        }
                        _ => reader.skip(),
                    }
                }
                LexemeKind::Word
            }
        };
        out.push(reader.collect(kind));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze() {
        let texts: &[Text] = &[
            Text::new("alpha.ahs", concat!(
                "A type System;\n",
                "B type Service{ # 😜🤖💩 }}",
            )),
            Text::new("beta.ahs", concat!(
                "C type Function {{}}\r\n",
                "D type Model\n{{{🤖}} c",
            )),
        ];
        let source = Source::new(texts);
        let source = Lexer::compile(&source).unwrap();

        // Check lexeme strings.
        assert_eq!(
            vec!["A", "type", "System", ";",
                 "B", "type", "Service", "{ # 😜🤖💩 }", "}",
                 "C", "type", "Function", "{{}}",
                 "D", "type", "Model", "{{{🤖}} c"],
            source.tree().iter()
                .map(|lexeme| lexeme.region().as_str())
                .collect::<Vec<_>>());

        // Check lexeme kinds.
        assert_eq!(
            vec![LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Semicolon,
                 LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Description, LexemeKind::BraceRight,
                 LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Description,
                 LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Description],
            source.tree().iter()
                .map(|lexeme| *lexeme.kind())
                .collect::<Vec<_>>());
    }
}