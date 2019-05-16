use ahfs_parse::Span;
use crate::spec::ServiceRef;

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