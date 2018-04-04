//! Lexical analysis utilities.
//!
//! This module provide various utilities for analyzing UTF-8 source strings.
//! Most significantly, it provides a [lexical analysis function][ana].
//!
//! [ana]: fn.analyze.html

mod lexeme;
mod lexeme_kind;
mod source;

pub use self::lexeme::Lexeme;
pub use self::lexeme_kind::LexemeKind;

use self::source::Source;

macro_rules! next_or_break {
    ($lexer:ident) => (match $lexer.next() { Some(ch) => ch, None => break });
}

/// Performs lexical analysis of given source string.
///
/// # Format
///
/// Skips ASCII whitespace and control characters. Treats the following
/// characters as delimiters: `# ;`. Any sequence of characters that is not
/// broken by any whitespace, control character or delimiter is treated as a
/// single lexeme.
///
/// ## Example
///
/// ```rust
/// let source = "R🤖BOT; from\nouter #space#!";
///
/// assert_eq!(
///     vec!["R🤖BOT", ";", "from", "outer", "#", "space", "#", "!"],
///     ::ahfs::compiler::lexer::analyze(source)
///         .iter()
///         .map(|lexeme| lexeme.extract(source))
///         .collect::<Vec<_>>());
/// ```
pub fn analyze<'a>(source: &'a str) -> Vec<Lexeme> {
    let mut source = Source::new(source);
    let mut lexemes = Vec::new();
    let mut ch: char;
    'outer: loop {
        ch = next_or_break!(source);

        if is_control(ch) {
            source.discard();
            continue 'outer;
        }

        loop {
            lexemes.push(source.collect(match ch {
                '#' => LexemeKind::Hash,
                ';' => LexemeKind::Semicolon,
                _ => { break; }
            }));
            continue 'outer;
        }

        loop {
            ch = next_or_break!(source);
            if is_control(ch) || is_delimiter(ch) {
                source.undo();
                break;
            }
        }
        lexemes.push(source.collect(LexemeKind::Word));
    }
    return lexemes;

    #[inline]
    fn is_control(ch: char) -> bool {
        match ch {
            '\x00'...' ' | '\x7f' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_delimiter(ch: char) -> bool {
        match ch {
            '#' | ';' => true,
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
            "B type Service;\n",
            "# Emojis 😜🤖💩!");

        let lexemes = super::analyze(SOURCE);

        // Check lexeme strings.
        assert_eq!(
            vec!["A", "type", "System", ";",
                 "B", "type", "Service", ";",
                 "#", "Emojis", "😜🤖💩!"],
            lexemes.iter()
                .map(|lexeme| lexeme.extract(SOURCE))
                .collect::<Vec<_>>());

        // Check lexeme kinds.
        assert_eq!(
            vec![LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Semicolon,
                 LexemeKind::Word, LexemeKind::Word, LexemeKind::Word,
                 LexemeKind::Semicolon,
                 LexemeKind::Hash, LexemeKind::Word, LexemeKind::Word],
            lexemes.iter()
                .map(|lexeme| *lexeme.kind())
                .collect::<Vec<_>>());
    }
}