//! Lexical analysis utilities.

use ahfs_parse::{Scanner, Token};
use crate::parser::TokenKind;

/// Creates a slice of `Tokens` from all characters accessible via given `scanner`.
pub fn all(mut scanner: Scanner) -> Vec<Token<TokenKind>> {
    let mut tokens = Vec::new();
    scan_all(&mut scanner, &mut tokens);
    tokens
}

#[inline]
fn scan_all<'a>(scanner: &mut Scanner<'a>, out: &mut Vec<Token<'a, TokenKind>>) -> Option<()> {
    let mut ch;
    loop {
        ch = scanner.next()?;

        if ch.is_whitespace() {
            scanner.discard();
            continue;
        }

        let name = match ch {
            '<' => TokenKind::AngleLeft,
            '>' => TokenKind::AngleRight,
            '{' => TokenKind::BraceLeft,
            '}' => TokenKind::BraceRight,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::ParenLeft,
            ')' => TokenKind::ParenRight,
            '[' => TokenKind::SquareLeft,
            ']' => TokenKind::SquareRight,
            ';' => TokenKind::Semicolon,
            '0' => scan_radix_number(scanner)?,
            '1'...'9' => scan_number(scanner)?,
            '+' | '-' => scan_number_or_symbol(scanner)?,
            '"' => scan_string(scanner)?,
            '/' => match scan_comment(scanner) {
                Some(name) => name,
                None => continue,
            },
            _ => scan_symbol(scanner, ch)?,
        };

        out.push(scanner.collect(name));
    }
}

#[inline]
fn scan_radix_number(scanner: &mut Scanner) -> Option<TokenKind> {
    let mut ch = scanner.next()?;
    match ch {
        'b' => loop {
            ch = scanner.next()?;
            match ch {
                '0'...'1' => continue,
                _ => break,
            }
        }
        'c' => loop {
            ch = scanner.next()?;
            match ch {
                '0'...'7' => continue,
                _ => break,
            }
        },
        'x' => loop {
            ch = scanner.next()?;
            match ch {
                '0'...'9' | 'A'...'F' | 'a'...'f' => continue,
                _ => break,
            }
        },
        '0'...'9' => {
            return scan_number(scanner);
        }
        _ => {}
    };
    scanner.unwind();
    Some(TokenKind::Integer)
}

fn scan_number(scanner: &mut Scanner) -> Option<TokenKind> {
    let mut is_float = false;
    let mut ch;

    // Integral.
    loop {
        ch = scanner.next()?;
        match ch {
            '0'...'9' => continue,
            _ => break,
        }
    }

    // Fraction.
    if ch == '.' {
        loop {
            ch = scanner.next()?;
            match ch {
                '0'...'9' => continue,
                _ => break,
            }
        }
        is_float = true;
    }

    // Exponent.
    if ch == 'E' || ch == 'e' {
        ch = scanner.next()?;
        if ch == '+' || ch == '-' {
            ch = scanner.next()?;
        }
        loop {
            match ch {
                '0'...'9' => {
                    ch = scanner.next()?;
                    continue;
                }
                _ => break,
            }
        }
        is_float = true;
    }

    scanner.unwind();

    Some(if is_float { TokenKind::Float } else { TokenKind::Integer })
}

#[inline]
fn scan_number_or_symbol(mut scanner: &mut Scanner) -> Option<TokenKind> {
    let ch = scanner.next()?;
    if ch >= '0' && ch <= '9' {
        scan_number(&mut scanner)
    } else if ch.is_whitespace() {
        scanner.unwind();
        Some(TokenKind::Error)
    } else {
        scan_symbol(&mut scanner, ch)
    }
}

#[inline]
fn scan_string(scanner: &mut Scanner) -> Option<TokenKind> {
    let mut ch;
    'outer: loop {
        ch = scanner.next()?;
        match ch {
            '"' => break Some(TokenKind::String),
            '\\' => {
                ch = scanner.next()?;
                match ch {
                    'u' => {
                        ch = scanner.next()?;
                        for _ in 0..4 {
                            match ch {
                                '0'...'9' |
                                'A'...'F' |
                                'a'...'f' => continue,
                                _ => break 'outer Some(TokenKind::Error),
                            }
                        }
                    }
                    _ => {}
                }
            }
            c if !c.is_control() => {}
            _ => break Some(TokenKind::Error),
        }
    }
}

#[inline]
fn scan_comment(scanner: &mut Scanner) -> Option<TokenKind> {
    let mut ch = scanner.next()?;
    match ch {
        '/' => {
            ch = scanner.next()?;
            let keep = ch == '/';
            loop {
                if ch == '\r' || ch == '\n' {
                    scanner.unwind();
                    break;
                }
                ch = scanner.next()?;
            }
            if keep {
                Some(TokenKind::Comment)
            } else {
                scanner.discard();
                return None;
            }
        }
        '*' => {
            ch = scanner.next()?;
            let keep = ch == '*';
            loop {
                if ch == '*' {
                    ch = scanner.next()?;
                    if ch == '/' {
                        break;
                    }
                }
                ch = scanner.next()?;
            }
            if keep {
                Some(TokenKind::Comment)
            } else {
                scanner.discard();
                return None;
            }
        }
        _ => {
            scanner.unwind();
            Some(TokenKind::Slash)
        }
    }
}

fn scan_symbol(scanner: &mut Scanner, mut ch: char) -> Option<TokenKind> {
    if !ch.is_alphabetic() && ch != '_' {
        return Some(TokenKind::Error);
    }
    loop {
        ch = scanner.next()?;
        if !(ch.is_alphanumeric() || ch == '_') {
            scanner.unwind();
            break;
        }
    }
    Some(match scanner.review() {
        // Keywords.
        "consumes" => TokenKind::Consumes,
        "implement" => TokenKind::Implement,
        "interface" => TokenKind::Interface,
        "method" => TokenKind::Method,
        "produces" => TokenKind::Produces,
        "property" => TokenKind::Property,
        "record" => TokenKind::Record,
        "service" => TokenKind::Service,
        "system" => TokenKind::System,
        "using" => TokenKind::Using,

        // Null.
        "null" => TokenKind::Null,

        // Booleans.
        "true" | "false" => TokenKind::Boolean,

        // Floats.
        "inf" | "+inf" | "-inf" | "NaN" => TokenKind::Float,

        // Errors.
        "+" | "-" => TokenKind::Error,

        // Identifier.
        _ => TokenKind::Identifier,
    })
}

#[cfg(test)]
mod tests {
    use ahfs_parse::Source;
    use super::*;

    #[test]
    fn all() {
        let source = Source {
            name: "alpha.ahfs".into(),
            body: concat!(
                "consumes implement interface method\n",
                "produces property record service system using\n",
                "\n",
                "<>{}:,()/[];\n",
                "\n",
                "null\n",
                "true false\n",
                "0 1 202 -30 +40\n",
                "50.0 6.1234 7.e+20 8e-10 1e9\n",
                "inf +inf -inf NaN\n",
                "\"Hello, World!\"\n",
                "\n",
                "IdentifierName smallCaps _underscore\n",
                "+ - * # ! ^ ~ ..\n",
                "/// This is a doc comment.\n",
                "/** This too! */\n",
                "// This is an ignored comment.\n",
                "/* This too! */\n",
            ).into()
        };
        let scanner = Scanner::new(&source);
        let tokens = super::all(scanner);

        // Check token strings.
        assert_eq!(
            vec![
                "consumes", "implement", "interface", "method",
                "produces", "property", "record", "service",
                "system", "using",
                "<", ">", "{", "}", ":", ",", "(", ")", "/", "[", "]", ";",
                "null",
                "true", "false",
                "0", "1", "202", "-30", "+40",
                "50.0", "6.1234", "7.e+20", "8e-10", "1e9",
                "inf", "+inf", "-inf", "NaN",
                "\"Hello, World!\"",
                "IdentifierName", "smallCaps", "_underscore",
                "+", "-", "*", "#", "!", "^", "~", ".", ".",
                "/// This is a doc comment.",
                "/** This too! */",
            ],
            tokens.iter().map(|item| item.span.as_str()).collect::<Vec<_>>()
        );

        // Check token kinds.
        assert_eq!(
            vec![
                TokenKind::Consumes, TokenKind::Implement, TokenKind::Interface, TokenKind::Method,
                TokenKind::Produces, TokenKind::Property, TokenKind::Record, TokenKind::Service,
                TokenKind::System, TokenKind::Using,
                TokenKind::AngleLeft, TokenKind::AngleRight,
                TokenKind::BraceLeft, TokenKind::BraceRight,
                TokenKind::Colon, TokenKind::Comma,
                TokenKind::ParenLeft, TokenKind::ParenRight,
                TokenKind::Slash,
                TokenKind::SquareLeft, TokenKind::SquareRight,
                TokenKind::Semicolon,
                TokenKind::Null,
                TokenKind::Boolean, TokenKind::Boolean,
                TokenKind::Integer, TokenKind::Integer, TokenKind::Integer,
                TokenKind::Integer, TokenKind::Integer,
                TokenKind::Float, TokenKind::Float, TokenKind::Float,
                TokenKind::Float, TokenKind::Float,
                TokenKind::Float, TokenKind::Float, TokenKind::Float, TokenKind::Float,
                TokenKind::String,
                TokenKind::Identifier, TokenKind::Identifier, TokenKind::Identifier,
                TokenKind::Error, TokenKind::Error, TokenKind::Error, TokenKind::Error,
                TokenKind::Error, TokenKind::Error, TokenKind::Error, TokenKind::Error,
                TokenKind::Error,
                TokenKind::Comment, TokenKind::Comment,
            ],
            tokens.iter().map(|item| item.kind).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn example1() {
        let source = Source {
            name: "example1.ahfs".into(),
            body: concat!(
                "/// Comment A.\n",
                "service MyService {\n",
                "    /// Comment B.\n",
                "    interface MyInterface {\n",
                "        /// Comment C.\n",
                "        method MyMethod(Argument): Result;\n",
                "    }\n",
                "}\n",
            ).into(),
        };
        let scanner = Scanner::new(&source);
        let tokens = super::all(scanner);

        // Check token strings.
        assert_eq!(
            vec![
                "/// Comment A.",
                "service", "MyService", "{",
                "/// Comment B.",
                "interface", "MyInterface", "{",
                "/// Comment C.",
                "method", "MyMethod", "(", "Argument", ")", ":", "Result", ";",
                "}",
                "}",
            ],
            tokens.iter().map(|item| item.span.as_str()).collect::<Vec<_>>()
        );

        // Check token kinds.
        assert_eq!(
            vec![
                TokenKind::Comment,
                TokenKind::Service, TokenKind::Identifier, TokenKind::BraceLeft,
                TokenKind::Comment,
                TokenKind::Interface, TokenKind::Identifier, TokenKind::BraceLeft,
                TokenKind::Comment,
                TokenKind::Method, TokenKind::Identifier, TokenKind::ParenLeft,
                TokenKind::Identifier, TokenKind::ParenRight, TokenKind::Colon,
                TokenKind::Identifier, TokenKind::Semicolon,
                TokenKind::BraceRight,
                TokenKind::BraceRight,
            ],
            tokens.iter().map(|item| item.kind).collect::<Vec<_>>(),
        );
    }
}