use arspec_parser::Span;
use super::{VerificationError, verify};

/// An enumerator type definition.
#[derive(Debug)]
pub struct Enum<'a> {
    /// Name of enum type.
    pub name: Span<'a>,

    /// Variant names.
    pub variants: Vec<EnumVariant<'a>>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> Enum<'a> {
    /// Create new enum type definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Enum {
            name,
            variants: Vec::new(),
            comment,
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

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> AsRef<str> for EnumVariant<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}