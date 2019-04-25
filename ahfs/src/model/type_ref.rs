use crate::parser;
use crate::source::Span;
use crate::model::Record;
use std::rc::Rc;

#[derive(Debug)]
pub enum TypeRef<'a> {
    Any {
        span: Span<'a>,
    },
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
    Option {
        span: Span<'a>,
        item_type: Box<TypeRef<'a>>,
    },
    Set {
        span: Span<'a>,
        item_type: Box<TypeRef<'a>>,
    },
    List {
        span: Span<'a>,
        item_type: Box<TypeRef<'a>>,
    },
    Map {
        span: Span<'a>,
        key_type: Box<TypeRef<'a>>,
        value_type: Box<TypeRef<'a>>,
    },
    Record {
        name: Span<'a>,
    },
}

impl<'a> TypeRef<'a> {
    pub fn try_from(type_ref: &parser::TypeRef<'a>) -> Option<Self> {
        let name = type_ref.name.as_str();
        let params = &type_ref.params as &[parser::TypeRef<'a>];
        Some(match (name, params) {
            ("Any", &[]) => TypeRef::Any {
                span: type_ref.name.clone(),
            },
            ("Void", &[]) => TypeRef::Void {
                span: type_ref.name.clone(),
            },
            ("Boolean", &[]) => TypeRef::Boolean {
                span: type_ref.name.clone(),
            },
            ("i8", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: true,
                bit_size: 8,
            },
            ("i16", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: true,
                bit_size: 16,
            },
            ("i32", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: true,
                bit_size: 32,
            },
            ("i64", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: true,
                bit_size: 64,
            },
            ("u8", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: false,
                bit_size: 8,
            },
            ("u16", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: false,
                bit_size: 16,
            },
            ("u32", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: false,
                bit_size: 32,
            },
            ("u64", &[]) => TypeRef::Integer {
                span: type_ref.name.clone(),
                signed: false,
                bit_size: 64,
            },
            ("f32", &[]) => TypeRef::Float {
                span: type_ref.name.clone(),
                bit_size: 32,
            },
            ("f64", &[]) => TypeRef::Float {
                span: type_ref.name.clone(),
                bit_size: 64,
            },
            ("Option", &[ref item_type]) => TypeRef::Option {
                span: type_ref.name.clone(),
                item_type: Box::new(TypeRef::try_from(item_type)?),
            },
            ("Set", &[ref item_type]) => TypeRef::Set {
                span: type_ref.name.clone(),
                item_type: Box::new(TypeRef::try_from(item_type)?),
            },
            ("List", &[ref item_type]) => TypeRef::List {
                span: type_ref.name.clone(),
                item_type: Box::new(TypeRef::try_from(item_type)?),
            },
            ("Map", &[ref key_type, ref value_type]) => TypeRef::Map {
                span: type_ref.name.clone(),
                key_type: Box::new(TypeRef::try_from(key_type)?),
                value_type: Box::new(TypeRef::try_from(value_type)?),
            },
            (_, &[]) => TypeRef::Record {
                name: type_ref.name.clone(),
            },
            _ => {
                return None;
            }
        })
    }
}
