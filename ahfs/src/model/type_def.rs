use crate::parser::Record;
use crate::source::Span;
use std::rc::Rc;

#[derive(Debug)]
pub enum TypeDef<'a> {
    Void {
        span: Span<'a>,
    },
    Boolean {
        span: Span<'a>,
    },
    Integer {
        span: Span<'a>,
        signed: bool,
        bit_size: u8,
    },
    Float {
        span: Span<'a>,
        bit_size: u8,
    },
    Record {
        name: Span<'a>,
    },
    Set {
        span: Span<'a>,
        item_type: Rc<TypeDef<'a>>,
    },
    List {
        span: Span<'a>,
        item_type: Rc<TypeDef<'a>>,
    },
    Map {
        span: Span<'a>,
        key_type: Rc<TypeDef<'a>>,
        value_type: Rc<TypeDef<'a>>,
    },
}

impl<'a> From<Record<'a>> for TypeDef<'a> {
    fn from(record: Record<'a>) -> Self {
        unimplemented!()
    }
}