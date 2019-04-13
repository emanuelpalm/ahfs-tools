mod method;
mod service;

pub use self::method::Method;
pub use self::service::Service;

use std::rc::Rc;

pub struct Model {
    functions: Box<[Rc<Method>]>,
    services: Box<[Service]>,
}
