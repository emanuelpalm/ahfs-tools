use arspec_parser::Span;
use super::{Attribute, VerificationError, verify};

/// An enumerator type definition.
#[derive(Debug)]
pub struct Enum<'a> {
    /// Name of enum type.
    pub name: Span<'a>,

    /// Variant names.
    pub variants: Vec<EnumVariant<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Enum<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        Enum {
            name,
            variants: Vec::new(),
            attributes,
        }
    }

    /// Asserts that this enum has no internal inconsistencies.
    pub fn verify(&self) -> Result<(), VerificationError> {
        verify::find_duplicate(&self.variants)
            .map(|dup| Err(VerificationError::EnumVariantDuplicate {
                original: dup.original.name.to_excerpt(),
                duplicate: dup.duplicate.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))
    }
}

impl<'a> AsRef<str> for Enum<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}

/// An enumerator variant definition.
#[derive(Debug)]
pub struct EnumVariant<'a> {
    /// Name of enum variant.
    pub name: Span<'a>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> EnumVariant<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        EnumVariant {
            name,
            attributes,
        }
    }
}

impl<'a> AsRef<str> for EnumVariant<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}