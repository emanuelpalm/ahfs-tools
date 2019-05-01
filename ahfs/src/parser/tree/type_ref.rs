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

    pub fn as_span(&self) -> Span<'a> {
        match self.params.last() {
            None => self.name.clone(),
            Some(param) => self.name.connect(param.as_span())
                .extend_while('>'),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_span().as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::source::{Range, Source};
    use super::*;

    #[test]
    fn as_str() {
        let source = Source::new("test", concat!(
            "Integer\n",
            "Option<Integer>\n",
            "Any<Integer, Option<Integer>>\n",
        ));
        let (a, b, c) = unsafe {
            let a = TypeRef {
                name: Span::new(&source, Range::new(0, 7)),
                params: Vec::new(),
            };
            let b = TypeRef {
                name: Span::new(&source, Range::new(8, 14)),
                params: vec![
                    TypeRef {
                        name: Span::new(&source, Range::new(15, 22)),
                        params: Vec::new(),
                    }
                ],
            };
            let c = TypeRef {
                name: Span::new(&source, Range::new(24, 27)),
                params: vec![
                    TypeRef {
                        name: Span::new(&source, Range::new(28, 35)),
                        params: Vec::new(),
                    },
                    TypeRef {
                        name: Span::new(&source, Range::new(37, 43)),
                        params: vec![
                            TypeRef {
                                name: Span::new(&source, Range::new(44, 51)),
                                params: Vec::new(),
                            }
                        ],
                    }
                ],
            };
            (a, b, c)
        };

        assert_eq!("Integer", a.as_str());
        assert_eq!("Option<Integer>", b.as_str());
        assert_eq!("Any<Integer, Option<Integer>>", c.as_str());
    }
}