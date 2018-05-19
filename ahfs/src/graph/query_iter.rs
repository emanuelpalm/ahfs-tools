use super::{Query, Triple};

/// A [`Query`][que] implementor wrapping an `Iterator`.
///
/// [que]: trait.Query.html
pub struct QueryIter<'a, 'b, I>
    where 'b: 'a, I: Iterator<Item=&'a Triple<'b>>
{
    iter: I,
    subject: Option<&'a str>,
    predicate: Option<&'a str>,
    object: Option<&'a str>,
}

impl<'a, 'b, I> QueryIter<'a, 'b, I>
    where 'b: 'a, I: Iterator<Item=&'a Triple<'b>>
{
    /// Creates new `QueryIter` from given `iter`.
    pub fn new<J>(iter: J) -> Self
        where J: Into<I>,
    {
        QueryIter {
            iter: iter.into(),
            subject: None,
            predicate: None,
            object: None,
        }
    }
}

impl<'a, 'b, I> Iterator for QueryIter<'a, 'b, I>
    where 'b: 'a, I: Iterator<Item=&'a Triple<'b>>
{
    type Item = &'a Triple<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if let Some(subject) = self.subject {
                if next.subject() != subject {
                    continue;
                }
            }
            if let Some(predicate) = self.predicate {
                if next.predicate() != predicate {
                    continue;
                }
            }
            if let Some(object) = self.object {
                if next.object() != object {
                    continue;
                }
            }
            return Some(next);
        }
    }
}

impl<'a, 'b, I> Query<'a, 'b> for QueryIter<'a, 'b, I>
    where 'b: 'a, I: Iterator<Item=&'a Triple<'b>>
{
    fn subject(mut self, subject: &'a str) -> Self {
        self.subject = Some(subject);
        self
    }

    fn predicate(mut self, predicate: &'a str) -> Self {
        self.predicate = Some(predicate);
        self
    }

    fn object(mut self, object: &'a str) -> Self {
        self.object = Some(object);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::graph::Graph;
    use ::parser;
    use ::source::Source;

    #[test]
    fn select() {
        let source: Source = concat!(
            "A type: Integer;\n",
            "B type: String;\n",
            "A value: 123;\n",
            "B value: \"Hello!\";\n").to_string().into_boxed_str().into();

        let triples = parser::parse(&source).unwrap();
        {
            let t: Vec<&Triple> = triples.query()
                .subject("A")
                .collect();

            assert_eq!(2, t.len());
            assert_eq!(t[0].subject(), "A");
            assert_eq!(t[0].predicate(), "type:");
            assert_eq!(t[0].object(), "Integer");
            assert_eq!(t[1].subject(), "A");
            assert_eq!(t[1].predicate(), "value:");
            assert_eq!(t[1].object(), "123");
        }
        {
            let t: Vec<&Triple> = triples.query()
                .subject("B")
                .predicate("type:")
                .collect();

            assert_eq!(1, t.len());
            assert_eq!(t[0].subject(), "B");
            assert_eq!(t[0].predicate(), "type:");
            assert_eq!(t[0].object(), "String");
        }
        {
            let t: Vec<&Triple> = triples.query()
                .predicate("value:")
                .object("123")
                .collect();

            assert_eq!(1, t.len());
            assert_eq!(t[0].subject(), "A");
            assert_eq!(t[0].predicate(), "value:");
            assert_eq!(t[0].object(), "123");
        }
        {
            let t: Vec<&Triple> = triples.query()
                .subject("C")
                .collect();

            assert_eq!(0, t.len());
        }
    }
}