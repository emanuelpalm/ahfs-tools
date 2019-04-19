//! Lexical analysis utilities.

use parser::{Name, Scanner, Token};
use source::Source;

/// Creates a slice of `Tokens` from given `source`.
pub fn analyze(source: &Source) -> Vec<Token> {
    let mut tokens = Vec::new();
    scan(source, &mut tokens);
    tokens
}

#[inline]
fn scan<'a>(source: &'a Source, out: &mut Vec<Token<'a>>) -> Option<()> {
    let mut scanner = Scanner::new(source);
    let mut ch;
    loop {
        ch = scanner.next()?;

        if ch.is_whitespace() {
            scanner.discard();
            continue;
        }

        let name = match ch {
            '<' => Name::AngleLeft,
            '>' => Name::AngleRight,
            '{' => Name::BraceLeft,
            '}' => Name::BraceRight,
            ':' => Name::Colon,
            ',' => Name::Comma,
            '(' => Name::ParenLeft,
            ')' => Name::ParenRight,
            '[' => Name::SquareLeft,
            ']' => Name::SquareRight,
            ';' => Name::Semicolon,
            '0' => scan_radix_number(&mut scanner)?,
            '1'...'9' => scan_number(&mut scanner)?,
            '+' | '-' => scan_number_or_symbol(&mut scanner)?,
            '"' => scan_string(&mut scanner)?,
            '/' => match scan_comment(&mut scanner) {
                Some(name) => name,
                None => continue,
            },
            _ => scan_symbol(&mut scanner, ch)?,
        };

        out.push(scanner.collect(name));
    }
}

#[inline]
fn scan_radix_number(scanner: &mut Scanner) -> Option<Name> {
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
    Some(Name::Integer)
}

fn scan_number(scanner: &mut Scanner) -> Option<Name> {
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

    Some(if is_float { Name::Float } else { Name::Integer })
}

#[inline]
fn scan_number_or_symbol(mut scanner: &mut Scanner) -> Option<Name> {
    let ch = scanner.next()?;
    if ch >= '0' && ch <= '9' {
        scan_number(&mut scanner)
    } else if ch.is_whitespace() {
        scanner.unwind();
        Some(Name::Error)
    } else {
        scan_symbol(&mut scanner, ch)
    }
}

#[inline]
fn scan_string(scanner: &mut Scanner) -> Option<Name> {
    let mut ch;
    'outer: loop {
        ch = scanner.next()?;
        match ch {
            '"' => break Some(Name::String),
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
                                _ => break 'outer Some(Name::Error),
                            }
                        }
                    }
                    _ => {}
                }
            }
            c if !c.is_control() => {}
            _ => break Some(Name::Error),
        }
    }
}

#[inline]
fn scan_comment(scanner: &mut Scanner) -> Option<Name> {
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
                Some(Name::Comment)
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
                Some(Name::Comment)
            } else {
                scanner.discard();
                return None;
            }
        }
        _ => {
            scanner.unwind();
            Some(Name::Slash)
        }
    }
}

fn scan_symbol(scanner: &mut Scanner, mut ch: char) -> Option<Name> {
    if !ch.is_alphabetic() && ch != '_' {
        return Some(Name::Error);
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
        "consumes" => Name::Consumes,
        "implement" => Name::Implement,
        "interface" => Name::Interface,
        "method" => Name::Method,
        "produces" => Name::Produces,
        "property" => Name::Property,
        "record" => Name::Record,
        "service" => Name::Service,
        "system" => Name::System,
        "using" => Name::Using,

        // Booleans.
        "true" | "false" => Name::Boolean,

        // Floats.
        "inf" | "+inf" | "-inf" | "NaN" => Name::Float,

        // Errors.
        "+" | "-" => Name::Error,

        // Identifier.
        _ => Name::Identifier,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let source = Source::new("alpha.ahfs", concat!(
            "consumes implement import interface method\n",
            "produces property record service system using\n",
            "\n",
            "<>{}:,()/[];\n",
            "\n",
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
        ));
        let tokens = super::analyze(&source);

        // Check token strings.
        assert_eq!(
            vec![
                "consumes", "implement", "interface", "method",
                "produces", "property", "record", "service",
                "system", "using",
                "<", ">", "{", "}", ":", ",", "(", ")", "/", "[", "]", ";",
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
            tokens.iter().map(|item| item.span().as_str()).collect::<Vec<_>>()
        );

        // Check token kinds.
        assert_eq!(
            vec![
                Name::Consumes, Name::Implement, Name::Interface, Name::Method,
                Name::Produces, Name::Property, Name::Record, Name::Service,
                Name::System, Name::Using,
                Name::AngleLeft, Name::AngleRight,
                Name::BraceLeft, Name::BraceRight,
                Name::Colon, Name::Comma,
                Name::ParenLeft, Name::ParenRight,
                Name::Slash,
                Name::SquareLeft, Name::SquareRight,
                Name::Semicolon,
                Name::Boolean, Name::Boolean,
                Name::Integer, Name::Integer, Name::Integer,
                Name::Integer, Name::Integer,
                Name::Float, Name::Float, Name::Float,
                Name::Float, Name::Float,
                Name::Float, Name::Float, Name::Float, Name::Float,
                Name::String,
                Name::Identifier, Name::Identifier, Name::Identifier,
                Name::Error, Name::Error, Name::Error, Name::Error,
                Name::Error, Name::Error, Name::Error, Name::Error,
                Name::Error,
                Name::Comment, Name::Comment,
            ],
            tokens.iter().map(|item| *item.name()).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn example1() {
        let source = Source::new("example1.ahfs", concat!(
            "/// Comment A.\n",
            "service MyService {\n",
            "    /// Comment B.\n",
            "    interface MyInterface {\n",
            "        /// Comment C.\n",
            "        method MyMethod(Argument): Result;\n",
            "    }\n",
            "}\n",
        ));
        let tokens = super::analyze(&source);

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
            tokens.iter().map(|item| item.span().as_str()).collect::<Vec<_>>()
        );

        // Check token kinds.
        assert_eq!(
            vec![
                Name::Comment,
                Name::Service, Name::Identifier, Name::BraceLeft,
                Name::Comment,
                Name::Interface, Name::Identifier, Name::BraceLeft,
                Name::Comment,
                Name::Method, Name::Identifier, Name::ParenLeft,
                Name::Identifier, Name::ParenRight, Name::Colon,
                Name::Identifier, Name::Semicolon,
                Name::BraceRight,
                Name::BraceRight,
            ],
            tokens.iter().map(|item| *item.name()).collect::<Vec<_>>(),
        );
    }
}