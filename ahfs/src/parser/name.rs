use std::fmt;

/// Classifies the [`Region`][region] identified by a [`Token`][token].
///
/// [region]: ../source/struct.Region.html
/// [token]: struct.Token.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    SquareLeft,
    SquareRight,
    Semicolon,

    // Literals.
    Boolean(bool),
    Integer,
    Float,
    String,

    // Keywords.
    Consumes,
    Implement,
    Import,
    Interface,
    Method,
    Produces,
    Record,
    Service,
    System,

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
            Name::SquareLeft => "[",
            Name::SquareRight => "]",
            Name::Semicolon => ";",

            Name::Boolean(_) => "Boolean",
            Name::Integer => "Integer",
            Name::Float => "Float",
            Name::String => "String",

            Name::Consumes => "consumes",
            Name::Implement => "implement",
            Name::Import => "import",
            Name::Interface => "interface",
            Name::Method => "method",
            Name::Produces => "produces",
            Name::Record => "record",
            Name::Service => "service",
            Name::System => "system",

            Name::Identifier => "Identifier",
            Name::Comment => "Comment",
            Name::Error => "Error"
        })
    }
}