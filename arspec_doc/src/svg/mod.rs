pub mod enum_;
pub mod record;
pub mod service;
pub mod system;

use crate::fonts;
use std::io;

pub trait Encode<M = Vector>
    where M: Size
{
    fn encode<W>(&self, offset: Vector, measurement: M, w: &mut W) -> io::Result<()>
        where W: io::Write;

    fn measure(&self) -> M;
}

pub trait Size {
    fn size(&self) -> Vector;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Size for Vector {
    #[inline]
    fn size(&self) -> Vector {
        *self
    }
}

/// Creates complete XML/SVG file for given SVG element.
pub fn render<E, M, W>(element: &E, w: &mut W) -> io::Result<()>
    where E: Encode<M>,
          M: Size,
          W: io::Write,
{
    let measurements = element.measure();
    let size = measurements.size();

    write!(w, concat!(
        "<svg width=\"{}px\" height=\"{}px\"",
        " xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
        "<g font-family=\"{}\">",
    ), size.x, size.y, fonts::SANS.name)?;

    element.encode(Vector::default(), measurements, w)?;

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
