use arspec_parser::Span;
use crate::spec::{Attribute, TypeRef};

/// An abstract service definition.
#[derive(Debug)]
pub struct Service<'a> {
    /// Service name.
    pub name: Span<'a>,

    /// Any service interface definitions.
    pub methods: Vec<ServiceMethod<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Service<'a> {
    /// Create new abstract service definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        Service {
            name,
            methods: Vec::new(),
            attributes,
        }
    }
}

impl<'a> AsRef<str> for Service<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}

/// Abstract service interface method definition.
#[derive(Debug)]
pub struct ServiceMethod<'a> {
    /// Name of method.
    pub name: Span<'a>,

    /// Method input type, unless no input is accepted.
    pub input: Option<TypeRef<'a>>,

    /// Method output type, unless no output is provided.
    pub output: Option<TypeRef<'a>>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> ServiceMethod<'a> {
    /// Create new abstract service interface method definition.
    #[inline]
    pub fn new(name: Span<'a>, attributes: Vec<Attribute<'a>>) -> Self {
        ServiceMethod {
            name,
            input: None,
            output: None,
            attributes,
        }
    }
}

/// A named abstract [`Service`][srv] reference.
///
/// [srv]: struct.Service.html
#[derive(Debug)]
pub struct ServiceRef<'a> {
    /// Name of referred [`Service`](struct.Service.html).
    pub name: Span<'a>,

    /// Any attributes.
    pub attributes: Vec<Attribute<'a>>,
}