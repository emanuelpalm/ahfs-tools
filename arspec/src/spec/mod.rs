pub mod parser;

mod enum_;
mod implement;
mod primitive;
mod property;
mod record;
mod service;
mod system;
mod type_ref;
mod value;
mod verify;

pub use self::enum_::{Enum, EnumVariant};
pub use self::implement::{Implement, ImplementInterface, ImplementMethod};
pub use self::primitive::Primitive;
pub use self::property::Property;
pub use self::record::{Record, RecordEntry};
pub use self::service::{Service, ServiceInterface, ServiceMethod, ServiceRef};
pub use self::system::System;
pub use self::type_ref::TypeRef;
pub use self::value::Value;

use arspec_parser::Excerpt;
use std::fmt;

/// An Arrowhead Framework specification collection.
#[derive(Debug, Default)]
pub struct Specification<'a> {
    /// Enumerator type definitions.
    pub enums: Vec<Enum<'a>>,

    /// Service implementation definitions.
    pub implementations: Vec<Implement<'a>>,

    /// Primitive type definitions.
    pub primitives: Vec<Primitive<'a>>,

    /// Record type definitions.
    pub records: Vec<Record<'a>>,

    /// Abstract service definitions.
    pub services: Vec<Service<'a>>,

    /// System definitions.
    pub systems: Vec<System<'a>>,
}

impl<'a> Specification<'a> {
    pub fn verify(&self) -> Result<(), VerificationError> {
        for enum_ in &self.enums {
            enum_.verify()?;
        }
        verify::find_duplicate(&self.enums)
            .map(|dup| Err(VerificationError::NameDuplicate {
                name_type: "enum",
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.primitives)
            .map(|dup| Err(VerificationError::NameDuplicate {
                name_type: "primitive",
                duplicate: dup.duplicate.definition.name.to_excerpt(),
                original: dup.original.definition.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.records)
            .map(|dup| Err(VerificationError::NameDuplicate {
                name_type: "record",
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.services)
            .map(|dup| Err(VerificationError::NameDuplicate {
                name_type: "service",
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.systems)
            .map(|dup| Err(VerificationError::NameDuplicate {
                name_type: "system",
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum VerificationError {
    NameDuplicate {
        name_type: &'static str,
        duplicate: Excerpt,
        original: Excerpt,
    },
}

impl<'a> fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VerificationError::NameDuplicate { ref name_type, ref duplicate, ref original } => {
                write!(
                    f,
                    concat!(
                        "`{}` specification with the name `{}` already exists.\n",
                        "Duplicate located at:\n",
                        "{}",
                        "Originally defined at:\n",
                        "{}",
                    ),
                    name_type,
                    original.as_str(),
                    duplicate,
                    original
                )
            }
        }
    }
}