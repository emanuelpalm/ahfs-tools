use source::Span;

#[derive(Debug)]
pub struct Tree<'a> {
    pub imports: Vec<Import<'a>>,
    pub services: Vec<Service<'a>>,
    pub systems: Vec<System<'a>>,
}

impl<'a> Tree<'a> {
    #[inline]
    pub fn new() -> Self {
        Tree {
            imports: Vec::new(),
            services: Vec::new(),
            systems: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Import<'a> {
    pub name: Span<'a>,
    pub comment: Option<Span<'a>>,
}

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

#[derive(Debug)]
pub struct System<'a> {
    pub name: Span<'a>,
    pub consumes: Vec<ServiceRef<'a>>,
    pub produces: Vec<ServiceRef<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> System<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        System {
            name,
            consumes: Vec::new(),
            produces: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct TypeRef<'a> {
    pub name: Span<'a>,
    pub params: Vec<TypeRef<'a>>,
}

impl<'a> TypeRef<'a> {
    #[inline]
    pub fn new(name: Span<'a>) -> Self {
        TypeRef {
            name,
            params: Vec::new(),
        }
    }
}