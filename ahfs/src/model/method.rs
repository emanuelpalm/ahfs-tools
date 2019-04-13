pub struct Method {
    name: Box<str>,
}

impl Method {
    #[inline]
    pub fn new<N>(name: N) -> Self
        where N: Into<Box<str>>,
    {
        Method { name: name.into() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}