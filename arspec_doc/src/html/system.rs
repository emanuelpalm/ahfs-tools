use arspec::spec::System;
use crate::svg;
use std::io;
use super::Encode;

impl<'a: 'b, 'b> Encode for &'b System<'a> {
    fn encode<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        svg::render(self, true, w)
    }

    #[inline]
    fn name(&self) -> &str {
        self.name.as_str()
    }
}