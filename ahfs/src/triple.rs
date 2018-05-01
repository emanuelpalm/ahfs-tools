use super::{Name, Token};
use super::source::{Range, Region, Text};

/// A specification triple.
///
/// Contains [`Region`s](../source/struct.Region.html) for a `subject`, a
/// `predicate`, an `object`, and an optional `description`.
#[derive(Debug, Eq, PartialEq)]
pub struct Triple<'a> {
    text: &'a Text<'a>,
    subject: Range,
    predicate: Range,
    object: Range,
    description: Range,
}

impl<'a> Triple<'a> {
    /// It is the responsibility of the caller to ensure given
    /// [`Range`s](type.Range.html) (`subject`, `predicate`, `object`) refer
    /// valid UTF-8 bounds within the same [`Text`](../source/struct.Text.html).
    #[doc(hidden)]
    #[inline]
    pub unsafe fn new<R, L>(subject: R, predicate: R, object: R, end: L) -> Self
        where R: Into<Range>,
              L: Into<Token<'a>>
    {
        let end = end.into();
        Triple {
            text: end.region().text(),
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            description: match *end.name() {
                Name::Description => end.into(),
                _ => 0..0,
            },
        }
    }

    /// `Triple` subject.
    #[inline]
    pub fn subject(&self) -> Region<'a> {
        unsafe { Region::new(self.text, self.subject.clone()) }
    }

    /// `Triple` predicate.
    #[inline]
    pub fn predicate(&self) -> Region<'a> {
        unsafe { Region::new(self.text, self.predicate.clone()) }
    }

    /// `Triple` object.
    #[inline]
    pub fn object(&self) -> Region<'a> {
        unsafe { Region::new(self.text, self.object.clone()) }
    }

    /// `Triple` description.
    #[inline]
    pub fn description(&self) -> Option<Region<'a>> {
        if self.description.start == self.description.end {
            return None;
        }
        Some(unsafe { Region::new(self.text, self.description.clone()) })
    }
}