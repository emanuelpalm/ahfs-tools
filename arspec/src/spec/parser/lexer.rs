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

        let class = match ch {
            '<' => Class::AngleLeft,
            '>' => Class::AngleRight,
            '@' => Class::At,
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
            '/' => match scan_comment_or_slash(scanner) {
                Some(slash) => slash,
                None => continue,
            },
            _ => scan_symbol(scanner, ch)?,
        };

        out.push(scanner.collect(class));
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
        Some(Class::InvalidSymbolChar)
    } else {
        scan_symbol(&mut scanner, ch)
    }
}

#[inline]
fn scan_string(scanner: &mut Scanner) -> Option<Class> {
    let mut class = Class::String;
    let mut ch;
    'outer: loop {
        ch = scanner.next()?;
        match ch {
            '"' => {
                return Some(class);
            }
            '\\' => {
                ch = scanner.next()?;
                match ch {
                    '"' | '\\' | 'n' | 'r' | 't' => {}
                    'u' => {
                        let mut i = 4;
                        while i > 0 {
                            ch = scanner.next()?;
                            i -= 1;
                            match ch {
                                '0'...'9' |
                                'A'...'F' |
                                'a'...'f' => continue,
                                _ => {
                                    if ch == '"' {
                                        scanner.unwind();
                                    }
                                    class = Class::InvalidStringEscape;
                                },
                            }
                        }
                    }
                    _ => {
                        class = Class::InvalidStringEscape;
                    }
                }
            }
            _ => {},
        }
    }
}

#[inline]
fn scan_comment_or_slash(scanner: &mut Scanner) -> Option<Class> {
    let mut ch = scanner.next()?;
    match ch {
        '/' => {
            ch = scanner.next()?;
            loop {
                if ch == '\r' || ch == '\n' {
                    scanner.unwind();
                    break;
                }
                ch = scanner.next()?;
            }
        }
        '*' => {
            ch = scanner.next()?;
            loop {
                if ch == '*' {
                    ch = scanner.next()?;
                    if ch == '/' {
                        break;
                    }
                }
                ch = scanner.next()?;
            }
        }
        _ => {
            scanner.unwind();
            return Some(Class::Slash);
        }
    }
    scanner.discard();
    None
}

fn scan_symbol(scanner: &mut Scanner, mut ch: char) -> Option<Class> {
    if !ch.is_alphabetic() && ch != '_' {
        return Some(Class::InvalidSymbolChar);
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
        "enum" => Class::Enum,
        "implement" => Class::Implement,
        "interface" => Class::Interface,
        "method" => Class::Method,
        "primitive" => Class::Primitive,
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
        "+" | "-" => Class::InvalidSymbolChar,

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
                "consumes enum implement interface method\n",
                "produces property record service system using\n",
                "\n",
                "<>{}:,()/[];\n",
                "\n",
                "null\n",
                "true false\n",
                "0 1 202 -30 +40\n",
                "50.0 6.1234 7.e+20 8e-10 1e9\n",
                "inf +inf -inf NaN\n",
                "\"Hello, World!\" \"\\uBad\" \"\\uFree\"\n",
                "\"123\\uXYZ456\"\n",
                "\n",
                "IdentifierName smallCaps _underscore\n",
                "+ - * # ! ^ ~ ..\n",
                "/// This is an ignored doc comment.\n",
                "/** This too! */\n",
                "// This is an ignored plain comment.\n",
                "/* This too! */\n",
            ).into(),
        };
        let scanner = Scanner::new(&source);
        let tokens = super::scan(scanner);

        // Check token strings.
        assert_eq!(
            vec![
                "consumes", "enum", "implement", "interface", "method",
                "produces", "property", "record", "service",
                "system", "using",
                "<", ">", "{", "}", ":", ",", "(", ")", "/", "[", "]", ";",
                "null",
                "true", "false",
                "0", "1", "202", "-30", "+40",
                "50.0", "6.1234", "7.e+20", "8e-10", "1e9",
                "inf", "+inf", "-inf", "NaN",
                "\"Hello, World!\"", "\"\\uBad\"", "\"\\uFree\"",
                "\"123\\uXYZ456\"",
                "IdentifierName", "smallCaps", "_underscore",
                "+", "-", "*", "#", "!", "^", "~", ".", ".",
            ],
            tokens.iter().map(|item| item.span.as_str()).collect::<Vec<_>>()
        );

        // Check token classes.
        assert_eq!(
            vec![
                Class::Consumes, Class::Enum, Class::Implement, Class::Interface, Class::Method,
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
                Class::String, Class::InvalidStringEscape, Class::InvalidStringEscape,
                Class::InvalidStringEscape,
                Class::Identifier, Class::Identifier, Class::Identifier,
                Class::InvalidSymbolChar, Class::InvalidSymbolChar, Class::InvalidSymbolChar,
                Class::InvalidSymbolChar, Class::InvalidSymbolChar, Class::InvalidSymbolChar,
                Class::InvalidSymbolChar, Class::InvalidSymbolChar, Class::InvalidSymbolChar,
            ],
            tokens.iter().map(|item| item.class).collect::<Vec<_>>(),
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
                "service", "MyService", "{",
                "interface", "MyInterface", "{",
                "method", "MyMethod", "(", "Argument", ")", ":", "Result", ";",
                "}",
                "}",
            ],
            tokens.iter().map(|item| item.span.as_str()).collect::<Vec<_>>()
        );

        // Check token classes.
        assert_eq!(
            vec![
                Class::Service, Class::Identifier, Class::BraceLeft,
                Class::Interface, Class::Identifier, Class::BraceLeft,
                Class::Method, Class::Identifier, Class::ParenLeft,
                Class::Identifier, Class::ParenRight, Class::Colon,
                Class::Identifier, Class::Semicolon,
                Class::BraceRight,
                Class::BraceRight,
            ],
            tokens.iter().map(|item| item.class).collect::<Vec<_>>(),
        );
    }
}