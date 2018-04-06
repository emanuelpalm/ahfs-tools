mod lexeme;
mod lexeme_kind;
mod source;

pub use self::lexeme::Lexeme;
pub use self::lexeme_kind::LexemeKind;

use self::source::Source;

macro_rules! next_or_break {
    ($lexer:ident) => (match $lexer.next() { Some(ch) => ch, None => break });
}

/// Turns given source string into vector of [`Lexemes`](struct.Lexeme.html).
pub fn analyze<'a>(source: &'a str) -> Vec<Lexeme> {
    let mut source = Source::new(source);
    let mut lexemes = Vec::new();
    'outer: loop {
        let mut c = next_or_break!(source);

        if is_control(c) {
            source.discard();
            continue 'outer;
        }

        if c == b'{' {
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
            let lexeme = source.collect(LexemeKind::Description);
            lexemes.push(lexeme.shrink(left_braces, right_braces));
            source.undo();
            continue 'outer;
        }

        let kind = match c {
            b';' => LexemeKind::Semicolon,
            b'}' => LexemeKind::BraceRight,
            _ => {
                loop {
                    c = next_or_break!(source);
                    if is_control(c) || is_delimiter(c) {
                        source.undo();
                        break;
                    }
                }
                LexemeKind::Word
            }
        };
        lexemes.push(source.collect(kind));
    }
    return lexemes;

    #[inline]
    fn is_control(c: u8) -> bool {
        match c {
            b'\0'...b' ' | 0x7f => true,
            _ => false,
        }
    }

    #[inline]
    fn is_delimiter(c: u8) -> bool {
        match c {
            b';' | b'{' | b'}' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LexemeKind;

    #[test]
    fn analyze() {
        const SOURCE: &'static str = concat!(
            "A type System;\n",
            "B type Service{ # 😜🤖💩 }}");

        let lexemes = super::analyze(SOURCE);

        // Check lexeme strings.
        assert_eq!(
            vec!["A", "type", "System", ";",
                 "B", "type", "Service", " # 😜🤖💩 ", "}"],
            lexemes.iter()
                .map(|lexeme| lexeme.extract(SOURCE))
                .collect::<Vec<_>>());

        // Check lexeme kinds.
        assert_eq!(
            vec![LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Semicolon,
                 LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Description, LexemeKind::BraceRight],
            lexemes.iter()
                .map(|lexeme| *lexeme.kind())
                .collect::<Vec<_>>());
    }
}