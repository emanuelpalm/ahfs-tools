use super::{Name, Token};
use ::source::{Range, Region, Text};

/// A specification triple.
///
/// # Usage
///
/// Identifies a `subject`, a `predicate`, an `object` and an optional
/// `description` within some [`Source`][src] code [`Text`][txt]. These fields
/// are permitted to have any kinds of values, meaning it is up to the user of
/// the type to interpret the meaning a given `Triple`. Generally, however, the
/// `subject` should name an _entity_ being subject of the triple, the
/// `object` should name an _entity_ associated with the `subject`, while the
/// `predicate` identifies the nature of the association. The `description`, if
/// available, describes the association in human language.
///
/// # Examples
///
/// The following are examples of potentially relevant triples in the syntax
/// parsed by the library [`parse`][prs] function:
///
/// ```ahfs
/// Orchestrator type System;
/// Orchestrator consumes ServiceDiscovery {
///     The ServiceDiscovery service is consumed to allow the Orchestrator to
///     make itself findable by other services.
/// }
/// Orchestrator produces Orchestration;
/// ```
///
/// [prs]: parser/fn.parse.html
/// [src]: source/struct.Source.html
/// [txt]: source/struct.Text.html
#[derive(Debug, Eq, PartialEq)]
pub struct Triple<'a> {
    text: &'a Text,
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