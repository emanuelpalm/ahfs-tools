use arspec_parser::Span;
use crate::spec::{Property, Value};

/// Specifies how to implement a named [`Service`][srv].
///
/// [srv]: struct.Service.html
#[derive(Debug)]
pub struct Implement<'a> {
    /// Name of implemented service.
    pub name: Span<'a>,

    /// Name of communication protocol to use.
    pub protocol: Span<'a>,

    /// Name of payload encoding to use.
    pub encoding: Span<'a>,

    /// Any implementation properties.
    pub properties: Vec<Property<'a>>,

    /// Any interface implementation definitions.
    pub interfaces: Vec<ImplementInterface<'a>>,

    /// Any documentation comment.
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

/// Specifies how to implement a named [`ServiceInterface`][irf].
///
/// [irf]: struct.ServiceInterface.html
#[derive(Debug)]
pub struct ImplementInterface<'a> {
    /// Name of implemented [`ServiceInterface`](struct.ServiceInterface.html).
    pub name: Span<'a>,

    /// Any interface method implementations.
    pub methods: Vec<ImplementMethod<'a>>,

    /// Any interface properties.
    pub properties: Vec<Property<'a>>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> ImplementInterface<'a> {
    /// Create new [`ServiceInterface`][irf] implementation definition.
    ///
    /// [irf]: struct.ServiceInterface.html
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

/// Specifies how to implement a named [`ServiceMethod`][met].
///
/// [met]: struct.ServiceMethod.html
#[derive(Debug)]
pub struct ImplementMethod<'a> {
    /// Name of implemented [`ServiceMethod`](struct.ServiceMethod.html).
    pub name: Span<'a>,

    /// Table of method specification data.
    pub data: Vec<(Span<'a>, Value<'a>)>,

    /// Any documentation comment.
    pub comment: Option<Span<'a>>,
}

impl<'a> ImplementMethod<'a> {
    /// Create new [`ServiceMethod`][irf] implementation definition.
    ///
    /// [irf]: struct.ServiceMethod.html
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ImplementMethod {
            name,
            data: Vec::new(),
            comment,
        }
    }
}