macro_rules! include_font {
    ($file:tt) => (include!(concat!(env!("OUT_DIR"), $file)));
}
static FONT_MONO: Font<'static> = include_font!("/font_mono.rs");
static FONT_SANS: Font<'static> = include_font!("/font_sans.rs");
static FONT_SANS_BOLD: Font<'static> = include_font!("/font_sans_bold.rs");
static FONT_SANS_ITALIC: Font<'static> = include_font!("/font_sans_italic.rs");

/// Identifies one particular font glyph.
pub type GlyphIndex = u16;

/// An unscaled font.
pub struct Font<'a> {
    name: &'a str,

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
    /// Default monospaced font.
    #[inline]
    pub fn mono() -> &'static Font<'static> {
        &FONT_MONO
    }

    /// Default sans serif font.
    #[inline]
    pub fn sans() -> &'static Font<'static> {
        &FONT_SANS
    }

    /// Default sans serif bold font.
    #[inline]
    pub fn sans_bold() -> &'static Font<'static> {
        &FONT_SANS_BOLD
    }

    /// Default sans serif italic font.
    #[inline]
    pub fn sans_italic() -> &'static Font<'static> {
        &FONT_SANS_ITALIC
    }

    /// Font name.
    #[inline]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Font ascender, in font units.
    #[inline]
    pub fn ascender(&self) -> i16 {
        self.ascender
    }

    /// Font descender, in font units.
    #[inline]
    pub fn descender(&self) -> i16 {
        self.descender
    }

    /// Font line gap, in font units.
    #[inline]
    pub fn line_gap(&self) -> i16 {
        self.line_gap
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

    /// Determines advance width, in font units, of glyph identified by `gi`.
    #[inline]
    pub fn advance_width_of(&self, gi: GlyphIndex) -> u16 {
        self.advance_widths[gi as usize]
    }

    /// Determines kerning width, in font units, between the glyphs identified
    /// by `a` and `b`. Note that the order of `a` and `b` is significant.
    pub fn kerning_between(&self, a: GlyphIndex, b: GlyphIndex) -> i16 {
        if let Some(kernings) = self.kernings {
            if a as usize >= kernings.len() {
                return 0;
            }
            let sublist = kernings[a as usize];
            match sublist.binary_search_by(|item| item.0.cmp(&b)) {
                Ok(index) => sublist[index].1,
                Err(_) => 0,
            }
        } else {
            0
        }
    }

    /// Determines line height in font units.
    #[inline]
    pub fn line_height(&self) -> i16 {
        self.ascender - self.descender + self.line_gap
    }

    /// Determines width, in font units, of given line of characters.
    #[inline]
    pub fn line_width_of(&self, line: &str) -> i32 {
        line.chars().fold((0, 0), |(mut width, last), ch| {
            let gi = self.glyph_index_of(ch);
            width += self.advance_width_of(gi) as i32 + self.kerning_between(last, gi) as i32;
            (width, gi)
        }).0
    }

    /// Creates scaled variant of font, with glyphs of given pixel size.
    pub fn scale<'b: 'a>(&'b self, size_px: f32) -> FontScaled<'a, 'b> {
        FontScaled {
            font: self,
            line_height: self.line_height() as f32,
            scale: size_px / (self.units_per_em as f32),
        }
    }

    /// Length, in font units, of both the width and height of the canonical
    /// glyph square.
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

/// A scaled font.
pub struct FontScaled<'a, 'b: 'a> {
    font: &'b Font<'a>,
    line_height: f32,
    scale: f32,
}

impl<'a, 'b: 'a> FontScaled<'a, 'b> {
    /// Font ascender, in pixels.
    #[inline]
    pub fn ascender(&self) -> f32 {
        self.font.ascender() as f32 * self.scale
    }

    /// Font descender, in pixels.
    #[inline]
    pub fn descender(&self) -> f32 {
        self.font.descender() as f32 * self.scale
    }

    /// Font line gap, in pixels.
    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.font.line_gap() as f32 * self.scale
    }

    /// Determines advance width, in pixels, of glyph identified by `gi`.
    #[inline]
    pub fn advance_width_of(&self, gi: GlyphIndex) -> f32 {
        self.font.advance_width_of(gi) as f32 * self.scale
    }

    /// Determines kerning width, in pixels, between the glyphs identified
    /// by `a` and `b`. Note that the order of `a` and `b` is significant.
    pub fn kerning_between(&self, a: GlyphIndex, b: GlyphIndex) -> f32 {
        self.font.kerning_between(a, b) as f32 * self.scale
    }

    /// Determines line height in pixels.
    #[inline]
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// Determines width, in pixels, of given line of characters.
    #[inline]
    pub fn line_width_of(&self, line: &str) -> f32 {
        self.font.line_width_of(line) as f32 * self.scale
    }
}