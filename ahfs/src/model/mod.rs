mod function;
mod service;

pub use self::function::Function;
pub use self::service::Service;

use std::rc::Rc;

pub struct Model {
    functions: Box<[Rc<Function>]>,
    services: Box<[Service]>,
}
