use arspec_parser::Span;
use super::VerificationError;

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
        for v0 in &self.variants {
            let mut count = 0;
            for v1 in &self.variants {
                if v0.name == v1.name {
                    count += 1;
                    if count > 1 {
                        return Err(VerificationError::EnumVariantDuplicate {
                            name: self.name.as_str().into(),
                            variant: v1.name.to_excerpt(),
                        });
                    }
                }
            }
        }
        Ok(())
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
