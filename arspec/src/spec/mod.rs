pub mod parser;

mod implement;
mod property;
mod record;
mod service;
mod system;
mod type_ref;
mod value;

pub use self::implement::{Implement, ImplementInterface, ImplementMethod};
pub use self::property::Property;
pub use self::record::{Record, RecordEntry};
pub use self::service::{Service, ServiceInterface, ServiceMethod, ServiceRef};
pub use self::system::System;
pub use self::type_ref::TypeRef;
pub use self::value::Value;

/// An Arrowhead Framework specification collection.
#[derive(Debug, Default)]
pub struct Specification<'a> {
    /// Service implementation definitions.
    pub implementations: Vec<Implement<'a>>,

    /// Record type definitions.
    pub records: Vec<Record<'a>>,

    /// Abstract service definitions.
    pub services: Vec<Service<'a>>,

    /// System definitions.
    pub systems: Vec<System<'a>>,
}
