use std::fmt;

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// [span]: ../../source/struct.Span.html
/// [token]: ../token/struct.Token.html
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Name {
    // Delimiters.
    AngleLeft,
    AngleRight,
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
    Boolean,
    Integer,
    Float,
    String,

    // Keywords.
    Consumes,
    Implement,
    Interface,
    Method,
    Produces,
    Property,
    Record,
    Service,
    System,
    Using,

    // Other.
    Identifier,
    Comment,
    Error,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Name::AngleLeft => "<",
            Name::AngleRight => ">",
            Name::BraceLeft => "{",
            Name::BraceRight => "}",
            Name::Colon => ":",
            Name::Comma => ",",
            Name::ParenLeft => "(",
            Name::ParenRight => ")",
            Name::Slash => "/",
            Name::SquareLeft => "[",
            Name::SquareRight => "]",
            Name::Semicolon => ";",

            Name::Boolean => "Boolean",
            Name::Integer => "Integer",
            Name::Float => "Float",
            Name::String => "String",

            Name::Consumes => "consumes",
            Name::Implement => "implement",
            Name::Interface => "interface",
            Name::Method => "method",
            Name::Produces => "produces",
            Name::Property => "property",
            Name::Record => "record",
            Name::Service => "service",
            Name::System => "system",
            Name::Using => "using",

            Name::Identifier => "Identifier",
            Name::Comment => "Comment",
            Name::Error => "Error",
        })
    }
}