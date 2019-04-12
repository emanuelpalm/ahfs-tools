pub struct Function {
    name: Box<str>,
    label: Box<str>,
}

impl Function {
    #[inline]
    pub fn new<N, L>(name: N, label: L) -> Self
        where N: Into<Box<str>>,
              L: Into<Box<str>>,
    {
        Function { name: name.into(), label: label.into() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn label(&self) -> &str {
        &self.label
    }
}