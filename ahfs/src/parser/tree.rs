use source::Region;

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
    pub name: Region<'a>,
}

#[derive(Debug)]
pub struct Service<'a> {
    pub name: Region<'a>,
    pub interfaces: Vec<ServiceInterface<'a>>,
    pub comment: Option<Region<'a>>,
}

impl<'a> Service<'a> {
    #[inline]
    pub fn new(name: Region<'a>, comment: Option<Region<'a>>) -> Self {
        Service {
            name,
            interfaces: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ServiceInterface<'a> {
    pub name: Region<'a>,
    pub methods: Vec<ServiceMethod<'a>>,
    pub comment: Option<Region<'a>>,
}

impl<'a> ServiceInterface<'a> {
    #[inline]
    pub fn new(name: Region<'a>, comment: Option<Region<'a>>) -> Self {
        ServiceInterface {
            name,
            methods: Vec::new(),
            comment,
        }
    }
}

#[derive(Debug)]
pub struct ServiceMethod<'a> {
    pub name: Region<'a>,
    pub input: Option<TypeRef<'a>>,
    pub output: Option<TypeRef<'a>>,
    pub comment: Option<Region<'a>>,
}

impl<'a> ServiceMethod<'a> {
    #[inline]
    pub fn new(name: Region<'a>, comment: Option<Region<'a>>) -> Self {
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
    pub name: Region<'a>,
    pub comment: Option<Region<'a>>,
}

#[derive(Debug)]
pub struct System<'a> {
    pub name: Region<'a>,
    pub consumes: Vec<ServiceRef<'a>>,
    pub produces: Vec<ServiceRef<'a>>,
    pub comment: Option<Region<'a>>,
}

impl<'a> System<'a> {
    #[inline]
    pub fn new(name: Region<'a>, comment: Option<Region<'a>>) -> Self {
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
    pub name: Region<'a>,
    pub params: Vec<TypeRef<'a>>,
}

impl<'a> TypeRef<'a> {
    #[inline]
    pub fn new(name: Region<'a>) -> Self {
        TypeRef {
            name,
            params: Vec::new(),
        }
    }
}