use crate::parser::TypeRef;
use crate::source::Span;

#[derive(Debug)]
pub struct Service<'a> {
    pub name: Span<'a>,
    pub interfaces: Vec<ServiceInterface<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> Service<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Service {
            name,
            interfaces: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ServiceInterface<'a> {
    pub name: Span<'a>,
    pub methods: Vec<ServiceMethod<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> ServiceInterface<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        ServiceInterface {
            name,
            methods: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ServiceMethod<'a> {
    pub name: Span<'a>,
    pub input: Option<TypeRef<'a>>,
    pub output: Option<TypeRef<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> ServiceMethod<'a> {
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

#[derive(Debug)]
pub struct ServiceRef<'a> {
    pub name: Span<'a>,
    pub comment: Option<Span<'a>>,
}