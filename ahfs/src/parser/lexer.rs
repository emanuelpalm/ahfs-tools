//! Lexical analysis utilities.

use super::{Name, Scanner, Token};
use ::source::{Source, Text};

/// Creates a slice of `Tokens` from given `source`.
pub fn analyze(source: &Source) -> Box<[Token]> {
    let mut tokens = Vec::new();
    for text in source.texts() {
        analyze_text(text, &mut tokens);
    }
    tokens.into_boxed_slice()
}

macro_rules! next {
    ($source:expr) => {
        match $source.next() {
            Some(c) => c,
            None => { return; }
        }
    };
}

fn analyze_text<'a>(text: &'a Text, out: &mut Vec<Token<'a>>) {
    let mut reader = Scanner::new(text);
    let mut c: char;

    'outer: loop {
        c = next!(reader);

        // Whitespace.
        if c.is_whitespace() {
            reader.discard();
            continue 'outer;
        }

        let name = match c {
            // Delimiters.
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

            // Integer or Float.
            '+' | '-' | '0'...'9' => {
                if c == '0' {
                    c = next!(reader);
                    match c {
                        'b' => loop {
                            c = next!(reader);
                            match c {
                                '0'...'1' => continue,
                                _ => break,
                            }
                        }
                        'c' => loop {
                            c = next!(reader);
                            match c {
                                '0'...'7' => continue,
                                _ => break,
                            }
                        },
                        'x' => loop {
                            c = next!(reader);
                            match c {
                                '0'...'9' | 'A'...'F' | 'a'...'f' => continue,
                                _ => break,
                            }
                        },
                        _ => {}
                    }
                    reader.undo();
                    Name::Integer
                } else {
                    loop {
                        c = next!(reader);
                        match c {
                            '0'...'9' => continue,
                            _ => break,
                        }
                    }
                    let mut is_float = false;
                    if c == '.' {
                        loop {
                            c = next!(reader);
                            match c {
                                '0'...'9' => continue,
                                _ => break,
                            }
                        }
                        is_float = true;
                    }
                    if c == 'E' || c == 'e' {
                        c = next!(reader);
                        if c == '+' || c == '-' {
                            c = next!(reader);
                        }
                        loop {
                            c = next!(reader);
                            match c {
                                '0'...'9' => continue,
                                _ => break,
                            }
                        }
                        is_float = true;
                    }
                    reader.undo();
                    if is_float {
                        Name::Float
                    } else {
                        Name::Integer
                    }
                }
            }

            // String.
            '"' => {
                loop {
                    c = next!(reader);
                    match c {
                        '"' => break Name::String,
                        '\\' => {
                            c = next!(reader);
                            match c {
                                'u' => {
                                    c = next!(reader);
                                    for i in 0..4 {
                                        match c {
                                            '0'...'9' |
                                            'A'...'F' |
                                            'a'...'f' => continue,
                                            _ => break,
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        c if !c.is_control() => {}
                        _ => break Name::Error,
                    }
                }
            }

            // Comment.
            '/' => {
                c = next!(reader);
                match c {
                    '/' => {
                        c = next!(reader);
                        let keep = c == '/';
                        loop {
                            if c == '\r' || c == '\n' {
                                reader.undo();
                                break;
                            }
                            c = next!(reader);
                        }
                        if keep {
                            Name::Comment
                        } else {
                            reader.discard();
                            continue 'outer;
                        }
                    }
                    '*' => {
                        c = next!(reader);
                        let keep = c == '*';
                        loop {
                            if c == '*' {
                                c = next!(reader);
                                if c == '/' {
                                    break;
                                }
                            }
                            c = next!(reader);
                        }
                        if keep {
                            Name::Comment
                        } else {
                            reader.discard();
                            continue 'outer;
                        }
                    }
                    _ => Name::Error,
                }
            }

            x if x.is_alphabetic() || x == '_' => {
                loop {
                    c = next!(reader);
                    if !(c.is_alphanumeric() || c == '_') {
                        reader.undo();
                        break;
                    }
                }
                match reader.review() {
                    // Keywords.
                    "consumes" => Name::Consumes,
                    "implement" => Name::Implement,
                    "import" => Name::Import,
                    "interface" => Name::Interface,
                    "method" => Name::Method,
                    "produces" => Name::Produces,
                    "record" => Name::Record,
                    "service" => Name::Service,
                    "system" => Name::System,

                    // Boolean.
                    "true" => Name::Boolean(true),
                    "false" => Name::Boolean(false),

                    // Identifier.
                    _ => Name::Identifier,
                }
            }

            _ => Name::Error,
        };
        out.push(reader.collect(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze() {
        let texts = vec![
            Text::new("alpha.ahfs", concat!(
                "consumes implement import interface method\n",
                "produces record service system\n",
                "\n",
                "<>{}:,()[];\n",
                "\n",
                "true false\n",
                "0 1 202 -30 +40 50.0 6.12 7e+20 8e-10 1e9\n",
                "\"Hello, World!\"\n",
                "\n",
                "IdentifierName smallCaps _underscore\n",
            )),
            Text::new("beta.ahfs", concat!(
                "/// This is a doc comment.\n",
                "/** This too! */\n",
                "// This is an ignored comment.\n",
                "/* This too! */\n",
            )),
        ];
        let source = Source::new(texts);
        let tokens = super::analyze(&source);

        // Check token strings.
        assert_eq!(
            vec!["consumes", "implement", "import", "interface",
                 "method", "produces", "record", "service", "system",
                 "<", ">", "{", "}", ":", ",", "(", ")", "[", "]", ";",
                 "true", "false",
                 "0", "1", "202", "-30", "+40", "50.0", "6.12", "7e+20",
                 "8e-10", "1e9",
                 "\"Hello, World!\"",
                 "IdentifierName", "smallCaps", "_underscore",
                 "/// This is a doc comment.",
                 "/** This too! */",
            ],
            tokens.iter().map(|item| item.region().as_str()).collect::<Vec<_>>()
        );

        // Check token kinds.
        assert_eq!(
            vec![Name::Consumes, Name::Implement, Name::Import, Name::Interface,
                 Name::Method, Name::Produces, Name::Record, Name::Service,
                 Name::System,
                 Name::AngleLeft, Name::AngleRight,
                 Name::BraceLeft, Name::BraceRight,
                 Name::Colon, Name::Comma,
                 Name::ParenLeft, Name::ParenRight,
                 Name::SquareLeft, Name::SquareRight,
                 Name::Semicolon,
                 Name::Boolean(true), Name::Boolean(false),
                 Name::Integer, Name::Integer, Name::Integer, Name::Integer,
                 Name::Integer, Name::Float, Name::Float, Name::Float,
                 Name::Float, Name::Float,
                 Name::String,
                 Name::Identifier, Name::Identifier, Name::Identifier,
                 Name::Comment, Name::Comment,
            ],
            tokens.iter().map(|item| *item.name()).collect::<Vec<_>>()
        );
    }
}