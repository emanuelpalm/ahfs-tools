use arspec_parser::{Range, Span};
use crate::spec::TypeRef;

/// An abstract service definition.
#[derive(Debug)]
pub struct Service<'a> {
    /// Service name.
    pub name: Span<'a>,

    /// Any service interface definitions.
    pub interfaces: Vec<ServiceInterface<'a>>,

    /// Any documentation comments.
    pub comment: Option<Span<'a>>,
}

impl<'a> Service<'a> {
    /// Create new abstract service definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Service {
            name,
            interfaces: Vec::new(),
            comment,
        }
    }
}

/// Abstract service interface definition.
#[derive(Debug)]
pub struct ServiceInterface<'a> {
    /// Name of interface.
    pub name: Span<'a>,

    /// Interface method definitions.
    pub methods: Vec<ServiceMethod<'a>>,

    /// Any documentation comments.
    pub comment: Option<Span<'a>>,
}

impl<'a> ServiceInterface<'a> {
    /// Create new abstract service interface definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ServiceInterface {
            name,
            methods: Vec::new(),
            comment,
        }
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

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> ServiceMethod<'a> {
    /// Create new abstract service interface method definition.
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ServiceMethod {
            name,
            input: None,
            output: None,
            comment,
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

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}