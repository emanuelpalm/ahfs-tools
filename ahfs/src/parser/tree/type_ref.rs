use crate::source::Span;

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