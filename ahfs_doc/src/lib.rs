pub trait Document {
    fn size(&self) -> Vector;
    fn render(&self, target: &mut [u8]);
}

pub trait Element {
    fn bounds(&self) -> Bounds;
    fn render(&self, target: &mut [u8]);
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds {
    pub offset: Vector,
    pub size: Vector,
}

#[derive(Debug)]
pub struct Text {
    pub font_size: f64,
    pub spans: Vec<TextSpan>,
}

impl Element for Text {
    fn bounds(&self) -> Bounds {
        unimplemented!()
    }

    fn render(&self, target: &mut [u8]) {
        unimplemented!()
    }
}

pub enum TextFont {
    SansSerif,
    Serif,
    Monospaced,
}

pub enum TextSize {
    Em(f64),
    Pixels(f64),
}

#[derive(Debug)]
pub struct TextSpan {
    pub style: TextStyle,
    pub data: String,
}

#[derive(Debug)]
pub enum TextStyle {
    Bold,
    BoldItalic,
    Italic,
    Regular,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}