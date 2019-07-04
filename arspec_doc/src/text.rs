use std::cmp;
use super::{Color, Font, Vector};

#[derive(Debug)]
pub struct Text {
    pub position: Vector,
    pub size: TextSize,
    pub lines: Vec<TextLine>,
    pub color: Color,
}

impl Text {
    #[inline]
    pub fn size(&self) -> (f32, f32) {
        let scale = self.size.scale();
        self.lines.iter().fold((0.0, 0.0), |(width, height), line| {
            let (width0, height0) = line.size(scale);
            (cmp::max(width, width0), height + height0)
        })
    }
}

#[derive(Debug)]
pub struct TextLine {
    pub spans: Vec<TextSpan>,
}

impl TextLine {
    #[inline]
    pub fn size(&self, scale: f32) -> (f32, f32) {
        self.spans.iter().fold((0.0, 0.0), |(width, height), span| {
            let (width0, height0) = span.size(scale);
            (width + width0, cmp::max(height, height0))
        })
    }
}

#[derive(Debug)]
pub enum TextSize {
    Footnote,
    Small,
    Medium,
    Large,
    Title,
    Px(f32),
}

impl TextSize {
    pub fn scale(&self) -> f32 {
        (match self {
            &TextSize::Footnote => 10.0,
            &TextSize::Small => 12.0,
            &TextSize::Medium => 16.0,
            &TextSize::Large => 24.0,
            &TextSize::Title => 36.0,
            &TextSize::Px(pixels) => pixels,
        })
    }
}

#[derive(Debug)]
pub struct TextSpan {
    pub style: TextStyle,
    pub data: String,
}

impl TextSpan {
    #[inline]
    pub fn size(&self, scale: f32) -> (f32, f32) {
        let font = self.style.font();
        let width = font.line_width_of(&self.data) * scale;
        let height = font.line_height() * scale;
        (width, height)
    }
}

#[derive(Debug)]
pub enum TextStyle {
    Mono,
    Sans,
    SansBold,
    SansItalic,
}

impl TextStyle {
    pub fn font(&self) -> &Font<'static> {
        match self {
            &TextStyle::Mono => Font::mono(),
            &TextStyle::Sans => Font::sans(),
            &TextStyle::SansBold => Font::sans_bold(),
            &TextStyle::SansItalic => Font::sans_italic(),
        }
    }
}
