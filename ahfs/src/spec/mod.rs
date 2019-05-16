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

use ahfs_parse::{Error, Text};
use self::parser::Class;
use std::path::Path;

/// A parse spec, derived from a single [`Source`][src].
///
/// [src]: ../../source/struct.Source.html
#[derive(Debug, Default)]
pub struct Specification<'a> {
    pub implementations: Vec<Implement<'a>>,
    pub records: Vec<Record<'a>>,
    pub services: Vec<Service<'a>>,
    pub systems: Vec<System<'a>>,
}
