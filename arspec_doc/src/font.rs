use std::fmt;

macro_rules! include_font {
    ($file:tt) => (include!(concat!(env!("OUT_DIR"), $file)));
}
static FONT_MONO: Font<'static> = include_font!("/font_mono.rs");
static FONT_SANS: Font<'static> = include_font!("/font_sans.rs");
static FONT_SANS_BOLD: Font<'static> = include_font!("/font_sans_bold.rs");
static FONT_SANS_ITALIC: Font<'static> = include_font!("/font_sans_italic.rs");
static FONT_ALL: &'static [&'static Font<'static>] = &[
    &FONT_MONO, &FONT_SANS, &FONT_SANS_BOLD, &FONT_SANS_ITALIC
];

/// Identifies one particular font glyph.
pub type GlyphIndex = u16;

/// An unscaled font.
pub struct Font<'a> {
    name: &'a str,
    style: FontStyle,
    weight: FontWeight,

    ascender: i16,
    descender: i16,
    line_gap: i16,
    units_per_em: u16,

    glyph_indexes_nonascii: &'a [u16],
    advance_widths: &'a [u16],
    kernings: Option<&'a [&'a [(GlyphIndex, i16)]]>,

    source: &'a [u8],
    source_name: &'a str,
}

impl<'a> Font<'a> {
    /// All default fonts.
    #[inline]
    pub fn all() -> &'static [&'static Font<'static>] {
        FONT_ALL
    }

    /// Default monospaced font.
    #[inline]
    pub fn mono() -> &'static Font<'static> {
        &FONT_MONO
    }

    /// Default sans-serif font.
    #[inline]
    pub fn sans() -> &'static Font<'static> {
        &FONT_SANS
    }

    /// Default sans-serif bold font.
    #[inline]
    pub fn sans_bold() -> &'static Font<'static> {
        &FONT_SANS_BOLD
    }

    /// Default sans-serif italic font.
    #[inline]
    pub fn sans_italic() -> &'static Font<'static> {
        &FONT_SANS_ITALIC
    }

    /// Font name.
    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Font style.
    #[inline]
    pub fn style(&self) -> &FontStyle {
        &self.style
    }

    /// Font weight.
    #[inline]
    pub fn weight(&self) -> &FontWeight {
        &self.weight
    }

    /// Font ascender, in EMs.
    #[inline]
    pub fn ascender(&self) -> f32 {
        self.ascender as f32 / self.units_per_em as f32
    }

    /// Font descender, in EMs.
    #[inline]
    pub fn descender(&self) -> f32 {
        self.descender as f32 / self.units_per_em as f32
    }

    /// Font line gap, in EMs.
    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.line_gap as f32 / self.units_per_em as f32
    }

    /// Acquires glyph index of `ch`.
    #[inline]
    pub fn glyph_index_of(&self, ch: char) -> GlyphIndex {
        if (ch as u32) < 127 {
            return ch as u32 as u16;
        }
        if ch as u32 > u16::max_value() as u32 {
            return 0;
        }
        match self.glyph_indexes_nonascii.binary_search(&(ch as u32 as u16)) {
            Ok(index) => index as GlyphIndex + 127,
            Err(_) => 0,
        }
    }

    /// Determines advance width, in EMs, of glyph identified by `gi`.
    #[inline]
    pub fn advance_width_of(&self, gi: GlyphIndex) -> f32 {
        self.advance_widths[gi as usize] as f32 / self.units_per_em as f32
    }

    /// Determines kerning width, in EMs, between the glyphs identified by `a`
    /// and `b`. Note that the order of `a` and `b` is significant.
    pub fn kerning_between(&self, a: GlyphIndex, b: GlyphIndex) -> f32 {
        if let Some(kernings) = self.kernings {
            if a as usize >= kernings.len() {
                return 0.0;
            }
            let sublist = kernings[a as usize];
            match sublist.binary_search_by(|item| item.0.cmp(&b)) {
                Ok(index) => sublist[index].1 as f32 / self.units_per_em as f32,
                Err(_) => 0.0,
            }
        } else {
            0.0
        }
    }

    /// Determines line height in EMs.
    #[inline]
    pub fn line_height(&self) -> f32 {
        (self.ascender - self.descender + self.line_gap) as f32 / self.units_per_em as f32
    }

    /// Determines width, in EMs, of given line of characters.
    ///
    /// Note that newlines in `line` are ignored.
    pub fn line_width_of(&self, line: &str) -> f32 {
        line.chars().fold((0.0, 0), |(mut width, last), ch| {
            let gi = self.glyph_index_of(ch);
            width += self.advance_width_of(gi) + self.kerning_between(last, gi);
            (width, gi)
        }).0
    }

    /// Length, in EMs, of both the width and height of the canonical glyph
    /// square.
    #[inline]
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    /// Raw font source file.
    #[inline]
    pub fn source(&self) -> &'a [u8] {
        self.source
    }

    /// File name of raw font source file.
    #[inline]
    pub fn source_name(&self) -> &'a str {
        self.source_name
    }
}

pub enum FontStyle {
    Normal,
    Italic,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            &FontStyle::Normal => "normal",
            &FontStyle::Italic => "italic",
        })
    }
}

pub enum FontWeight {
    Normal,
    Bold,
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            &FontWeight::Normal => "normal",
            &FontWeight::Bold => "bold",
        })
    }
}