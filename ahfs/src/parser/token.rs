use parser::Name;
use source::{Range, Region};

/// Identifies a typed [`Region`][reg] of some [`Source`][src] [`Text`][txt].
///
/// [reg]: ../source/struct.Region.html
/// [src]: ../source/struct.Source.html
/// [txt]: ../source/struct.Text.html
#[derive(Clone, Debug)]
pub struct Token<'a> {
    name: Name,
    region: Region<'a>,
}

impl<'a> Token<'a> {
    /// Creates new `Token` from given `name` and `region`.
    #[inline]
    pub fn new(name: Name, region: Region<'a>) -> Self {
        Token { name, region }
    }

    /// `Token` name.
    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// `Token` region.
    #[inline]
    pub fn region(&self) -> &Region<'a> {
        &self.region
    }

    /// `Token` region.
    #[inline]
    pub fn into_region(self) -> Region<'a> {
        self.region
    }
}

impl<'a> From<Token<'a>> for Range {
    #[inline]
    fn from(token: Token<'a>) -> Self {
        *token.region.range()
    }
}

impl<'a> From<&'a Token<'a>> for Range {
    #[inline]
    fn from(token: &'a Token<'a>) -> Self {
        token.region.range().clone()
    }
}
