pub mod record;
pub mod service;
pub mod system;

use crate::Font;
use std::io;

pub trait Encode {
    fn encode<W>(&self, size: Size, w: &mut W) -> io::Result<()>
        where W: io::Write;

    fn measure(&self) -> Size;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Creates complete XML/SVG file for given SVG element.
pub fn render<E, W>(element: &E, w: &mut W) -> io::Result<()>
    where E: Encode,
          W: io::Write,
{
    let size = element.measure();

    write!(w, concat!(
        "<svg width=\"{}px\" height=\"{}px\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
        "<g font-family=\"{}\">",
    ), size.width, size.height, Font::sans().name())?;

    element.encode(size, w)?;

    write!(w, "</g></svg>")
}

mod color {
    pub const ALPHA: &'static str = "#004676";
    pub const BETA: &'static str = "#006f99";
    pub const GAMMA: &'static str = "#8f3165";
    pub const RULER: &'static str = "#d0d0d0";
    pub const META: &'static str = "#666666";
    pub const TEXT: &'static str = "#333333";
}
