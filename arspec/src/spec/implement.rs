use arspec_parser::Span;
use crate::spec::{Property, Value};

/// Specifies how to implement a named [`Service`][srv].
///
/// [srv]: ../service/struct.Service.html
#[derive(Debug)]
pub struct Implement<'a> {
    pub name: Span<'a>,
    pub protocol: Span<'a>,
    pub encoding: Span<'a>,
    pub properties: Vec<Property<'a>>,
    pub interfaces: Vec<ImplementInterface<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> Implement<'a> {
    /// Create new service implementation specification.
    #[inline]
    pub fn new(
        name: Span<'a>,
        protocol: Span<'a>,
        encoding: Span<'a>,
        comment: Option<Span<'a>>,
    ) -> Self {
        Implement {
            name,
            protocol,
            encoding,
            properties: Vec::new(),
            interfaces: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ImplementInterface<'a> {
    pub name: Span<'a>,
    pub methods: Vec<ImplementMethod<'a>>,
    pub properties: Vec<Property<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> ImplementInterface<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ImplementInterface {
            name,
            methods: Vec::new(),
            properties: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ImplementMethod<'a> {
    pub name: Span<'a>,
    pub data: Vec<(Span<'a>, Value<'a>)>,
    pub comment: Option<Span<'a>>,
}

impl<'a> ImplementMethod<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ImplementMethod {
            name,
            data: Vec::new(),
            comment,
        }
    }
}