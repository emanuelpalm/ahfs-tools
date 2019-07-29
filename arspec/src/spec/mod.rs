pub mod parser;

mod attribute;
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

pub use self::attribute::Attribute;
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
    /// Performs _naive_ specification verification.
    ///
    /// TODO: Make this much more sophisticated. Add more passes.
    pub fn verify(&self) -> Result<(), VerificationError> {
        for enum_ in &self.enums {
            enum_.verify()?;
        }
        verify::find_duplicate(&self.enums)
            .map(|dup| Err(VerificationError::EnumNameDuplicate {
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        for implementation in &self.implementations {
            implementation.verify(self)?;
        }

        verify::find_duplicate(&self.primitives)
            .map(|dup| Err(VerificationError::PrimitiveNameDuplicate {
                duplicate: dup.duplicate.definition.name.to_excerpt(),
                original: dup.original.definition.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.records)
            .map(|dup| Err(VerificationError::RecordNameDuplicate {
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.services)
            .map(|dup| Err(VerificationError::ServiceNameDuplicate {
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        verify::find_duplicate(&self.systems)
            .map(|dup| Err(VerificationError::SystemNameDuplicate {
                duplicate: dup.duplicate.name.to_excerpt(),
                original: dup.original.name.to_excerpt(),
            }))
            .unwrap_or(Ok(()))?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum VerificationError {
    EnumNameDuplicate { duplicate: Excerpt, original: Excerpt },
    EnumVariantDuplicate { duplicate: Excerpt, original: Excerpt },
    InterfaceNotImplemented { interface: Excerpt, implementation: Excerpt },
    NoSuchInterfaceToImplement { service: Excerpt, interface: Excerpt },
    NoSuchServiceToImplement { service: Excerpt },
    PrimitiveNameDuplicate { duplicate: Excerpt, original: Excerpt },
    RecordNameDuplicate { duplicate: Excerpt, original: Excerpt },
    ServiceNameDuplicate { duplicate: Excerpt, original: Excerpt },
    SystemNameDuplicate { duplicate: Excerpt, original: Excerpt },
    UnknownServiceEncoding { encoding: Excerpt },
    UnknownServiceProtocol { protocol: Excerpt },
}

impl<'a> fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            &VerificationError::EnumNameDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "Enum", duplicate, original)
            }
            &VerificationError::EnumVariantDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "Enum variant", duplicate, original)
            }
            &VerificationError::InterfaceNotImplemented { ref interface, ref implementation } => {
                write!(
                    f,
                    concat!(
                        "Service interface not implemented.\n",
                        "The following service interface is defined:\n",
                        "{}\n",
                        "That service interface is, however, not provided by this implementation:\n",
                        "{}",
                    ),
                    interface, implementation,
                )
            }
            &VerificationError::NoSuchInterfaceToImplement { ref service, ref interface } => {
                write!(
                    f,
                    concat!(
                        "Superfluous interface implementation.\n",
                        "The following service interface definition is provided:\n",
                        "{}\n",
                        "However, no interface with that name is specified by the implemented service:\n",
                        "{}",
                    ),
                    interface, service,
                )
            }
            &VerificationError::NoSuchServiceToImplement { ref service } => {
                write!(f, "Cannot implement non-existing service.\n{}", service)
            }
            &VerificationError::PrimitiveNameDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "Primitive", duplicate, original)
            }
            &VerificationError::RecordNameDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "Record", duplicate, original)
            }
            &VerificationError::ServiceNameDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "Service", duplicate, original)
            }
            &VerificationError::SystemNameDuplicate { ref duplicate, ref original } => {
                write_name_duplicate_message(f, "System", duplicate, original)
            }
            &VerificationError::UnknownServiceEncoding { ref encoding } => {
                write!(
                    f,
                    concat!(
                        "Unknown service encoding. Valid encodings are `CBOR`, `JSON` and `XML`.",
                        "\n{}",
                    ),
                    encoding,
                )
            }
            &VerificationError::UnknownServiceProtocol { ref protocol } => {
                write!(
                    f,
                    concat!(
                        "Unknown service protocol. Valid protocol are `COAP`, `HTTP` and `MQTT`.",
                        "\n{}",
                    ),
                    protocol,
                )
            }
        };

        fn write_name_duplicate_message(
            f: &mut fmt::Formatter,
            name_type: &str,
            duplicate: &Excerpt,
            original: &Excerpt,
        ) -> fmt::Result
        {
            write!(
                f,
                concat!(
                        "{} named `{}` already exists.\n",
                        "Duplicate located at:\n",
                        "{}\n",
                        "Original located at:\n",
                        "{}",
                    ),
                name_type, original.as_str(), duplicate, original,
            )
        }
    }
}