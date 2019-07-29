use std::fmt;

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// [span]: ../../../arspec_parser/struct.Span.html
/// [token]: ../../../arspec_parser/struct.Token.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Class {
    // Delimiters.
    AngleLeft,
    AngleRight,
    At,
    BraceLeft,
    BraceRight,
    Colon,
    Comma,
    ParenLeft,
    ParenRight,
    Slash,
    SquareLeft,
    SquareRight,
    Semicolon,

    // Literals.
    Null,
    Boolean,
    Integer,
    Float,
    String,

    // Keywords.
    Consumes,
    Enum,
    Implement,
    Interface,
    Method,
    Primitive,
    Produces,
    Property,
    Record,
    Service,
    System,
    Using,

    // Other.
    Identifier,

    // Errors.
    InvalidStringEscape,
    InvalidStringChar,
    InvalidSymbolChar,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Class::AngleLeft => "<",
            Class::AngleRight => ">",
            Class::At => "@",
            Class::BraceLeft => "{",
            Class::BraceRight => "}",
            Class::Colon => ":",
            Class::Comma => ",",
            Class::ParenLeft => "(",
            Class::ParenRight => ")",
            Class::Slash => "/",
            Class::SquareLeft => "[",
            Class::SquareRight => "]",
            Class::Semicolon => ";",

            Class::Null => "null",
            Class::Boolean => "{Boolean}",
            Class::Integer => "{Integer}",
            Class::Float => "{Float}",
            Class::String => "{String}",

            Class::Consumes => "consumes",
            Class::Enum => "enum",
            Class::Implement => "implement",
            Class::Interface => "interface",
            Class::Method => "method",
            Class::Primitive => "primitive",
            Class::Produces => "produces",
            Class::Property => "property",
            Class::Record => "record",
            Class::Service => "service",
            Class::System => "system",
            Class::Using => "using",

            Class::Identifier => "{Identifier}",

            Class::InvalidStringEscape => "{InvalidStringEscape}",
            Class::InvalidStringChar => "{InvalidStringChar}",
            Class::InvalidSymbolChar => "{InvalidSymbolChar}",
        })
    }
}