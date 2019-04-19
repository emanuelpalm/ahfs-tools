use source::Span;

#[derive(Debug)]
pub struct Tree<'a> {
    pub implements: Vec<Implement<'a>>,
    pub imports: Vec<Import<'a>>,
    pub services: Vec<Service<'a>>,
    pub systems: Vec<System<'a>>,
}

impl<'a> Tree<'a> {
    #[inline]
    pub fn new() -> Self {
        Tree {
            implements: Vec::new(),
            imports: Vec::new(),
            services: Vec::new(),
            systems: Vec::new(),
        }
    }
}

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
    #[inline]
    pub fn new(name: Span<'a>, protocol: Span<'a>, encoding: Span<'a>, comment: Option<Span<'a>>) -> Self {
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

#[derive(Debug)]
pub struct Import<'a> {
    pub name: Span<'a>,
    pub comment: Option<Span<'a>>,
}

#[derive(Debug)]
pub struct Property<'a> {
    pub name: Span<'a>,
    pub value: Value<'a>,
    pub comment: Option<Span<'a>>,
}

#[derive(Debug)]
pub struct Record<'a> {
    pub name: Span<'a>,
    pub entries: Vec<RecordEntry<'a>>,
    pub comment: Option<Span<'a>>,
}

impl<'a> Record<'a> {
    #[inline]
    pub fn new(name: Span<'a>, comment: Option<Span<'a>>) -> Self {
        Record {
            name,
            entries: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct RecordEntry<'a> {
    pub name: Span<'a>,
    pub type_: TypeRef<'a>,
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

#[derive(Debug)]
pub enum Value<'a> {
    Null,
    Boolean(Span<'a>),
    Integer(Span<'a>),
    Float(Span<'a>),
    String(Span<'a>),
    List(Box<[Value<'a>]>),
    Map(Box<[(Span<'a>, Value<'a>)]>),
}