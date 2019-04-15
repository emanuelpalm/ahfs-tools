use source::Region;

#[derive(Debug)]
pub struct Tree<'a> {
    pub imports: Vec<Import<'a>>,
    pub systems: Vec<System<'a>>,
}

impl<'a> Tree<'a> {
    #[inline]
    pub fn new() -> Self {
        Tree {
            imports: Vec::new(),
            systems: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Import<'a> {
    pub name: Region<'a>,
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
    pub fn new(name: Region<'a>) -> Self {
        System {
            name,
            consumes: Vec::new(),
            produces: Vec::new(),
            comment: None,
        }
    }
}