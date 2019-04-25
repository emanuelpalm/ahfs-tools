mod record;
mod type_ref;

pub use self::record::{Record, RecordEntry};
pub use self::type_ref::TypeRef;

use crate::parser;
use std::collections::HashMap;
use std::iter::FromIterator;

/// A parse tree, derived from a single [`Source`][src].
///
/// [src]: ../../source/struct.Source.html
#[derive(Debug)]
pub struct Model<'a> {
    //pub implements: HashMap<&'a str, Implement<'a>>,
    pub records: HashMap<&'a str, TypeRef<'a>>,
    //pub services: HashMap<&'a str, Service<'a>>,
    //pub systems: HashMap<&'a str, System<'a>>,
}

impl<'a> From<parser::Tree<'a>> for Model<'a> {
    fn from(mut tree: parser::Tree<'a>) -> Self {
        unimplemented!();
        //return Model {
            //implements: ...,
            //records: {
            //    let pairs = tree.records.drain(..).map(|it| {
            //        (it.name.as_str(), Record::from(it))
            //    });
            //    HashMap::from_iter(pairs)
            //},
            //services: ...,
            //systems: ...,
        //};
    }
}