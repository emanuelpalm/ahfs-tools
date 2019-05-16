use std::fmt;

/// Classifies the [`Span`][span] identified by a [`Token`][token].
///
/// TODO: Fix markdown links.
///
/// [span]: ../../source/struct.Span.html
/// [token]: ../token/struct.Token.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
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
    Null,
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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            TokenKind::AngleLeft => "<",
            TokenKind::AngleRight => ">",
            TokenKind::BraceLeft => "{",
            TokenKind::BraceRight => "}",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::ParenLeft => "(",
            TokenKind::ParenRight => ")",
            TokenKind::Slash => "/",
            TokenKind::SquareLeft => "[",
            TokenKind::SquareRight => "]",
            TokenKind::Semicolon => ";",

            TokenKind::Null => "Null",
            TokenKind::Boolean => "Boolean",
            TokenKind::Integer => "Integer",
            TokenKind::Float => "Float",
            TokenKind::String => "String",

            TokenKind::Consumes => "consumes",
            TokenKind::Implement => "implement",
            TokenKind::Interface => "interface",
            TokenKind::Method => "method",
            TokenKind::Produces => "produces",
            TokenKind::Property => "property",
            TokenKind::Record => "record",
            TokenKind::Service => "service",
            TokenKind::System => "system",
            TokenKind::Using => "using",

            TokenKind::Identifier => "Identifier",
            TokenKind::Comment => "Comment",
            TokenKind::Error => "Error",
        })
    }
}