use super::{LexemeKind, Range, Region};

/// Identifies a typed [`Region`][reg] of some [`Source`][src] [`Text`][txt].
///
/// [reg]: ../source/struct.Region.html
/// [src]: ../source/struct.Source.html
/// [txt]: ../source/struct.Text.html
#[derive(Clone, Debug)]
pub struct Lexeme<'a> {
    kind: LexemeKind,
    region: Region<'a>,
}

impl<'a> Lexeme<'a> {
    /// Creates new `Lexeme` from given `kind` and `region`.
    #[inline]
    pub fn new(kind: LexemeKind, region: Region<'a>) -> Self {
        Lexeme { kind, region }
    }

    /// `Lexeme` kind.
    #[inline]
    pub fn kind(&self) -> &LexemeKind {
        &self.kind
    }

    /// `Lexeme` region.
    #[inline]
    pub fn region(&self) -> &Region<'a> {
        &self.region
    }
}

impl<'a> From<Lexeme<'a>> for Range {
    #[inline]
    fn from(lexeme: Lexeme<'a>) -> Self {
        lexeme.region.range().clone()
    }
}

impl<'a> From<&'a Lexeme<'a>> for Range {
    #[inline]
    fn from(lexeme: &'a Lexeme<'a>) -> Self {
        lexeme.region.range().clone()
    }
}