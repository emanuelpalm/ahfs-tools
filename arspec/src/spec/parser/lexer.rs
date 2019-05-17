//! Lexical analysis utilities.

use arspec_parser::{Scanner, Token};
use super::Class;

/// Create a slice of [`Tokens`][tok] from all characters accessible via given
/// [`scanner`][sca].
///
/// [sca]: ../../../arspec_parser/struct.Scanner.html
/// [tok]: ../../../arspec_parser/struct.Token.html
pub fn scan(mut scanner: Scanner) -> Vec<Token<Class>> {
    let mut tokens = Vec::new();
    scan_all(&mut scanner, &mut tokens);
    tokens
}

#[inline]
fn scan_all<'a>(scanner: &mut Scanner<'a>, out: &mut Vec<Token<'a, Class>>) -> Option<()> {
    let mut ch;
    loop {
        ch = scanner.next()?;

        if ch.is_whitespace() {
            scanner.discard();
            continue;
        }

        let name = match ch {
            '<' => Class::AngleLeft,
            '>' => Class::AngleRight,
            '{' => Class::BraceLeft,
            '}' => Class::BraceRight,
            ':' => Class::Colon,
            ',' => Class::Comma,
            '(' => Class::ParenLeft,
            ')' => Class::ParenRight,
            '[' => Class::SquareLeft,
            ']' => Class::SquareRight,
            ';' => Class::Semicolon,
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
fn scan_radix_number(scanner: &mut Scanner) -> Option<Class> {
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
    Some(Class::Integer)
}

fn scan_number(scanner: &mut Scanner) -> Option<Class> {
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

    Some(if is_float { Class::Float } else { Class::Integer })
}

#[inline]
fn scan_number_or_symbol(mut scanner: &mut Scanner) -> Option<Class> {
    let ch = scanner.next()?;
    if ch >= '0' && ch <= '9' {
        scan_number(&mut scanner)
    } else if ch.is_whitespace() {
        scanner.unwind();
        Some(Class::Error)
    } else {
        scan_symbol(&mut scanner, ch)
    }
}

#[inline]
fn scan_string(scanner: &mut Scanner) -> Option<Class> {
    let mut ch;
    'outer: loop {
        ch = scanner.next()?;
        match ch {
            '"' => break Some(Class::String),
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
                                _ => break 'outer Some(Class::Error),
                            }
                        }
                    }
                    _ => {}
                }
            }
            c if !c.is_control() => {}
            _ => break Some(Class::Error),
        }
    }
}

#[inline]
fn scan_comment(scanner: &mut Scanner) -> Option<Class> {
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
                Some(Class::Comment)
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
                Some(Class::Comment)
            } else {
                scanner.discard();
                return None;
            }
        }
        _ => {
            scanner.unwind();
            Some(Class::Slash)
        }
    }
}

fn scan_symbol(scanner: &mut Scanner, mut ch: char) -> Option<Class> {
    if !ch.is_alphabetic() && ch != '_' {
        return Some(Class::Error);
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
        "consumes" => Class::Consumes,
        "implement" => Class::Implement,
        "interface" => Class::Interface,
        "method" => Class::Method,
        "produces" => Class::Produces,
        "property" => Class::Property,
        "record" => Class::Record,
        "service" => Class::Service,
        "system" => Class::System,
        "using" => Class::Using,

        // Null.
        "null" => Class::Null,

        // Booleans.
        "true" | "false" => Class::Boolean,

        // Floats.
        "inf" | "+inf" | "-inf" | "NaN" => Class::Float,

        // Errors.
        "+" | "-" => Class::Error,

        // Identifier.
        _ => Class::Identifier,
    })
}

#[cfg(test)]
mod tests {
    use arspec_parser::Text;
    use super::*;

    #[test]
    fn all() {
        let source = Text {
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
        let tokens = super::scan(scanner);

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

        // Check token classes.
        assert_eq!(
            vec![
                Class::Consumes, Class::Implement, Class::Interface, Class::Method,
                Class::Produces, Class::Property, Class::Record, Class::Service,
                Class::System, Class::Using,
                Class::AngleLeft, Class::AngleRight,
                Class::BraceLeft, Class::BraceRight,
                Class::Colon, Class::Comma,
                Class::ParenLeft, Class::ParenRight,
                Class::Slash,
                Class::SquareLeft, Class::SquareRight,
                Class::Semicolon,
                Class::Null,
                Class::Boolean, Class::Boolean,
                Class::Integer, Class::Integer, Class::Integer,
                Class::Integer, Class::Integer,
                Class::Float, Class::Float, Class::Float,
                Class::Float, Class::Float,
                Class::Float, Class::Float, Class::Float, Class::Float,
                Class::String,
                Class::Identifier, Class::Identifier, Class::Identifier,
                Class::Error, Class::Error, Class::Error, Class::Error,
                Class::Error, Class::Error, Class::Error, Class::Error,
                Class::Error,
                Class::Comment, Class::Comment,
            ],
            tokens.iter().map(|item| item.kind).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn example1() {
        let source = Text {
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
        let tokens = super::scan(scanner);

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

        // Check token classes.
        assert_eq!(
            vec![
                Class::Comment,
                Class::Service, Class::Identifier, Class::BraceLeft,
                Class::Comment,
                Class::Interface, Class::Identifier, Class::BraceLeft,
                Class::Comment,
                Class::Method, Class::Identifier, Class::ParenLeft,
                Class::Identifier, Class::ParenRight, Class::Colon,
                Class::Identifier, Class::Semicolon,
                Class::BraceRight,
                Class::BraceRight,
            ],
            tokens.iter().map(|item| item.kind).collect::<Vec<_>>(),
        );
    }
}