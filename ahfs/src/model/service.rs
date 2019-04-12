use std::io;
use std::rc::Rc;
use super::Function;

pub struct Service {
    name: Box<str>,
    label: Box<str>,
    functions: Box<[Rc<Function>]>,
}

impl Service {
    #[inline]
    pub fn new<N, L, F>(name: N, label: L, functions: F) -> Self
        where N: Into<Box<str>>,
              L: Into<Box<str>>,
              F: Into<Box<[Rc<Function>]>>,
    {
        Service {
            name: name.into(),
            label: label.into(),
            functions: functions.into(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[inline]
    pub fn functions(&self) -> &[Rc<Function>] {
        &self.functions
    }
}
