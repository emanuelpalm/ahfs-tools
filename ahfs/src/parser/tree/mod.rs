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

/// A parse tree, derived from a single [`Source`][src].
///
/// [src]: ../../source/struct.Source.html
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Tree<'a> {
    pub implements: Vec<Implement<'a>>,
    pub records: Vec<Record<'a>>,
    pub services: Vec<Service<'a>>,
    pub systems: Vec<System<'a>>,
}

impl<'a> Tree<'a> {
    /// Create new empty parse tree.
    #[inline]
    pub fn new() -> Self {
        Tree {
            implements: Vec::new(),
            records: Vec::new(),
            services: Vec::new(),
            systems: Vec::new(),
        }
    }
}
