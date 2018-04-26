//! Lexical analysis utilities.

mod name;
mod scanner;
mod token;

pub use self::name::Name;
pub use self::token::Token;

use self::scanner::Scanner;
use super::source::{Range, Region, Source, Text};
use super::Tree;

/// A [`Tree`](../struct.Tree.html) of [`Token`s](struct.Token.html).
pub type TokenTree<'a> = Tree<'a, [Token<'a>]>;

/// Lexical analyzer.
///
/// Transforms a set of source texts into an array of
/// [`Token`s](struct.Token.html).
pub struct Lexer;

impl Lexer {
    /// Creates a slice of `Tokens` from given `source`.
    pub fn analyze<'a>(source: Source<'a>) -> TokenTree<'a> {
        let mut tokens = Vec::new();
        for text in source.texts() {
            analyze_text(text, &mut tokens);
        }
        TokenTree::new(source, tokens)
    }
}

macro_rules! next_or_break {
    ($source:expr) => (match $source.next() { Some(c) => c, None => break });
}
macro_rules! peek_or_break {
    ($source:expr) => (match $source.peek() { Some(c) => c, None => break });
}

fn analyze_text<'a>(text: &'a Text<'a>, out: &mut Vec<Token<'a>>) {
    let mut reader = Scanner::new(text);
    loop {
        let mut c = next_or_break!(reader);
        let kind = match c {

            // Whitespace.
            b'\0'...b' ' | 0x7f => {
                reader.discard();
                continue;
            }

            // Delimiter.
            b';' => Name::Semicolon,
            b'}' => Name::BraceRight,

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
                Name::Description
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
                Name::Word
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
        let tree = Lexer::analyze(Source::new(texts));
        let tokens = tree.root();

        // Check token strings.
        assert_eq!(
            vec!["A", "type", "System", ";",
                 "B", "type", "Service", "{ # 😜🤖💩 }", "}",
                 "C", "type", "Function", "{{}}",
                 "D", "type", "Model", "{{{🤖}} c"],
            tokens.iter()
                .map(|token| token.region().as_str())
                .collect::<Vec<_>>());

        // Check token kinds.
        assert_eq!(
            vec![Name::Word, Name::Word, Name::Word,
                 Name::Semicolon,
                 Name::Word, Name::Word, Name::Word,
                 Name::Description, Name::BraceRight,
                 Name::Word, Name::Word, Name::Word,
                 Name::Description,
                 Name::Word, Name::Word, Name::Word,
                 Name::Description],
            tokens.iter()
                .map(|token| *token.name())
                .collect::<Vec<_>>());
    }
}