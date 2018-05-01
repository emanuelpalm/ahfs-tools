use super::Source;

pub struct Tree<'a, T: ? Sized> {
    source: Source<'a>,
    root: Box<T>,
}

impl<'a, T: ? Sized> Tree<'a, T> {
    /// Creates new `Tree` from given `source` and `root`.
    #[inline]
    pub fn new<S, R>(source: S, root: R) -> Self
        where S: Into<Source<'a>>,
              R: Into<Box<T>>,
    {
        Tree { source: source.into(), root: root.into() }
    }

    /// [`Source`][src] from which `Tree` has been derived.
    ///
    /// [src]: ../source/struct.Source.html
    #[inline]
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// `Tree` root.
    #[inline]
    pub fn root(&self) -> &T {
        &self.root
    }
}

impl<'a, T: ? Sized> From<Tree<'a, T>> for Source<'a> {
    #[inline]
    fn from(tree: Tree<'a, T>) -> Self {
        tree.source
    }
}