use model::Method;

pub struct Service {
    name: Box<str>,
    methods: Box<[Method]>,
}

impl Service {
    #[inline]
    pub fn new<N, L, F>(name: N, label: L, methods: F) -> Self
        where N: Into<Box<str>>,
              L: Into<Box<str>>,
              F: Into<Box<[Method]>>,
    {
        Service {
            name: name.into(),
            methods: methods.into(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }
}
