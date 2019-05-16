use arspec_parser::{Span, Range};

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
        if self.params.len() == 0 {
            return self.name.clone();
        }
        let mut end = self.name.range.end;
        let mut chars = self.name.source.body[end..].chars();
        let mut height = 0;
        loop {
            match chars.next() {
                Some(ch) => {
                    end += ch.len_utf8();
                    match ch {
                        '<' => height += 1,
                        '>' => {
                            height -= 1;
                            if height <= 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                None => break,
            }
        }
        Span {
            source: self.name.source,
            range: Range { start: self.name.range.start, end },
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_span().as_str()
    }
}

#[cfg(test)]
mod tests {
    use arspec_parser::Text;
    use super::*;
    use std::ops;

    #[test]
    fn as_str() {
        let source = Text {
            name: "test".into(),
            body: concat!(
                "Integer\n",
                "Option<Integer>\n",
                "Any<Integer, Option<Integer>>\n",
                "Map<String, String>\n"
            ).into()
        };
        let span = |range: ops::Range<usize>| {
            Span { source: &source, range: range.into() }
        };
        let (a, b, c, d) = {
            let a = TypeRef {
                name: span(0..7),
                params: Vec::new(),
            };
            let b = TypeRef {
                name: span(8..14),
                params: vec![
                    TypeRef {
                        name: span(15..22),
                        params: Vec::new(),
                    }
                ],
            };
            let c = TypeRef {
                name: span(24..27),
                params: vec![
                    TypeRef {
                        name: span(28..35),
                        params: Vec::new(),
                    },
                    TypeRef {
                        name: span(37..43),
                        params: vec![
                            TypeRef {
                                name: span(44..51),
                                params: Vec::new(),
                            }
                        ],
                    }
                ],
            };
            let d = TypeRef {
                name: span(54..57),
                params: vec![
                    TypeRef {
                        name: span(66..72),
                        params: Vec::new(),
                    },
                    TypeRef {
                        name: span(58..64),
                        params: Vec::new(),
                    }
                ],
            };
            (a, b, c, d)
        };
        assert_eq!("Integer", a.as_str());
        assert_eq!("Option<Integer>", b.as_str());
        assert_eq!("Any<Integer, Option<Integer>>", c.as_str());
        assert_eq!("Map<String, String>", d.as_str());
    }
}