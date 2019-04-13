use source::Region;

pub struct Tree<'a> {
    pub imports: Box<[Import<'a>]>,
    pub systems: Box<[System<'a>]>,
}

pub struct Import<'a> {
    pub name: Region<'a>,
}

pub struct System<'a> {
    pub name: Region<'a>,
    pub consumes: Box<[Region<'a>]>,
    pub produces: Box<[Region<'a>]>,
    pub comment: Option<Region<'a>>,
}
