mod lexeme;
mod lexeme_kind;
mod source;

pub use self::lexeme::Lexeme;
pub use self::lexeme_kind::LexemeKind;

use self::source::Source;

macro_rules! next_or_break {
    ($source:expr) => (match $source.next() { Some(c) => c, None => break });
}
macro_rules! peek_or_break {
    ($source:expr) => (match $source.peek() { Some(c) => c, None => break });
}

/// Turns given source string into vector of [`Lexeme`s](struct.Lexeme.html).
pub fn analyze<'a>(source: &'a str) -> Vec<Lexeme<'a>> {
    let mut source = Source::new(source);
    let mut lexemes = Vec::new();
    loop {
        let mut c = next_or_break!(source);
        let kind = match c {

            // Whitespace.
            b'\0'...b' ' | 0x7f => {
                source.discard();
                continue;
            }

            // Delimiter.
            b';' => LexemeKind::Semicolon,
            b'}' => LexemeKind::BraceRight,

            // Description.
            b'{' => {
                let mut left_braces = 1;
                loop {
                    c = next_or_break!(source);
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
                    c = next_or_break!(source);
                }
                LexemeKind::Description
            }

            // Word.
            _ => {
                loop {
                    match peek_or_break!(source) {
                        b'\0'...b' ' | b';' | b'{' | b'}' | 0x7f => {
                            break;
                        }
                        _ => source.skip(),
                    }
                }
                LexemeKind::Word
            }
        };
        lexemes.push(source.collect(kind));
    }
    lexemes
}

#[cfg(test)]
mod tests {
    use super::LexemeKind;

    #[test]
    fn analyze() {
        const SOURCE: &'static str = concat!(
            "A type System;\n",
            "B type Service{ # 😜🤖💩 }}",
            "C type Function {{}}\r\n",
            "D type Model\n{{{🤖}} c");

        let lexemes = super::analyze(SOURCE);

        // Check lexeme strings.
        assert_eq!(
            vec!["A", "type", "System", ";",
                 "B", "type", "Service", "{ # 😜🤖💩 }", "}",
                 "C", "type", "Function", "{{}}",
                 "D", "type", "Model", "{{{🤖}} c"],
            lexemes.iter()
                .map(|lexeme| lexeme.as_str())
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
            lexemes.iter()
                .map(|lexeme| *lexeme.kind())
                .collect::<Vec<_>>());
    }
}