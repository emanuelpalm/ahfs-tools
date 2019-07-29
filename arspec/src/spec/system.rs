use arspec_parser::Span;
use crate::spec::{Attribute, ServiceRef};

/// System definition.
#[derive(Debug)]
pub struct System<'a> {
    /// Name of defined system.
    pub name: Span<'a>,

    /// The names of any [`Services`][srv] consumed by the system.
    ///
    /// [srv]: struct.Service.html
    pub consumes: Vec<ServiceRef<'a>>,

    /// The names of any [`Services`][srv] produced by the system.
    ///
    /// [srv]: struct.Service.html
    pub produces: Vec<ServiceRef<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> System<'a> {
    /// Create new system definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        System {
            name,
            consumes: Vec::new(),
            produces: Vec::new(),
            attributes,
        }
    }
}

impl<'a> AsRef<str> for System<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}
