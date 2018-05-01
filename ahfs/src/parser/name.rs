use std::fmt;

/// Classifies the [`Region`][region] identified by a [`Token`][token].
///
/// [region]: ../source/struct.Region.html
/// [token]: struct.Token.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Name {
    Semicolon,
    Description,
    BraceRight,
    Word,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Name::Semicolon => f.write_str(";"),
            Name::Description => f.write_str("{"),
            Name::BraceRight => f.write_str("}"),
            Name::Word => f.write_str("word"),
        }
    }
}