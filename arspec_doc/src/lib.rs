mod font;
mod text;

pub use self::font::{Font, GlyphIndex};
pub use self::text::{Text, TextLine, TextSize, TextSpan, TextStyle};

#[derive(Debug)]
pub struct Document {
    pub name: String,
    pub elements: Vec<Element>,
}

impl Document {
    pub fn render_svg(&self) {
    }
}

#[derive(Debug)]
pub enum Element {
    Rect(Rect),
    Text(Text),
}

#[derive(Debug)]
pub struct Rect {
    pub position: Vector,
    pub size: Vector,
    pub color: Color,
    pub radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}