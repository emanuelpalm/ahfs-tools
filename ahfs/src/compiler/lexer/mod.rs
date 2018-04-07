mod lexeme;
mod lexeme_kind;
mod source;

pub use self::lexeme::Lexeme;
pub use self::lexeme_kind::LexemeKind;

use self::source::Source;

/// Turns given source string into vector of [`Lexeme`s](struct.Lexeme.html).
pub fn analyze<'a>(source: &'a str) -> Vec<Lexeme> {
    let mut session = Session {
        source: Source::new(source),
        lexemes: Vec::new(),
    };
    session.document();
    session.lexemes
}

struct Session<'a> {
    source: Source<'a>,
    lexemes: Vec<Lexeme>,
}

macro_rules! read_or_break {
    ($lexer:expr) => (match $lexer.read() { Some(c) => c, None => break });
}

impl<'a> Session<'a> {
    fn document(&mut self) {
        loop {
            match read_or_break!(self.source) {
                b'\0'...b' ' | 0x7f => self.nothing(),
                b';' => self.delimiter(LexemeKind::Semicolon),
                b'{' => self.description(),
                b'}' => self.delimiter(LexemeKind::BraceRight),
                _ => self.word(),
            }
        }
    }

    fn nothing(&mut self) {
        self.source.next();
        self.source.discard();
    }

    fn delimiter(&mut self, kind: LexemeKind) {
        self.lexemes.push(self.source.collect(kind));
        self.nothing();
    }

    fn description(&mut self) {
        let mut c = 0u8;
        let mut left_braces = 1;
        loop {
            self.source.next();
            c = read_or_break!(self.source);
            if c != b'{' { break; }
            left_braces += 1;
        }
        let mut right_braces = 0;
        loop {
            if c == b'}' {
                right_braces += 1;
                if left_braces == right_braces {
                    self.source.next();
                    break;
                }
            } else {
                right_braces = 0;
            }
            self.source.next();
            c = read_or_break!(self.source);
        }
        let lexeme = self.source.collect(LexemeKind::Description);
        self.lexemes.push(lexeme.shrink(left_braces, right_braces));
    }

    fn word(&mut self) {
        loop {
            match read_or_break!(self.source) {
                b'\0'...b' ' | b';' | b'{' | b'}' | 0x7f => {
                    break;
                }
                _ => self.source.next(),
            }
        }
        self.lexemes.push(self.source.collect(LexemeKind::Word));
    }
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
            "D type Model\n{{{🤖}}");

        let lexemes = super::analyze(SOURCE);

        // Check lexeme strings.
        assert_eq!(
            vec!["A", "type", "System", "",
                 "B", "type", "Service", " # 😜🤖💩 ", "",
                 "C", "type", "Function", "",
                 "D", "type", "Model", "🤖"],
            lexemes.iter()
                .map(|lexeme| lexeme.extract(SOURCE))
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