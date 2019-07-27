use std::fmt;

/// Identifies one particular font glyph.
pub type GlyphIndex = u16;

/// A unsized font.
pub struct Font<'a> {
    pub name: &'a str,
    pub style: FontStyle,
    pub weight: FontWeight,

    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,

    /// Number of font size units per EM, where the canonical size of a glyph
    /// is exactly 1x1 EM.
    pub units_per_em: u16,

    pub glyph_indexes_nonascii: &'a [u16],
    pub advance_widths: &'a [u16],
    pub kernings: Option<&'a [&'a [(GlyphIndex, i16)]]>,

    /// Raw font source file.
    pub source: &'a [u8],

    /// File name of raw font source file.
    pub source_name: &'a str,
}

impl<'a> Font<'a> {
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

    /// Acquire glyph index of `ch`.
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

    /// Determine advance width, in EMs, of glyph identified by `gi`.
    #[inline]
    pub fn advance_width_of(&self, gi: GlyphIndex) -> f32 {
        self.advance_widths[gi as usize] as f32 / self.units_per_em as f32
    }

    /// Determine kerning width, in EMs, between the glyphs identified by `a`
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

    /// Determine font line height in EMs.
    #[inline]
    pub fn line_height(&self) -> f32 {
        (self.ascender - self.descender + self.line_gap) as f32
            / self.units_per_em as f32
    }

    /// Determine width, in EMs, of given line of characters.
    ///
    /// Note that newlines in `line` are ignored.
    pub fn line_width_of(&self, line: &str) -> f32 {
        line.chars().fold((0.0, 0), |(mut width, last), ch| {
            let gi = self.glyph_index_of(ch);
            width += self.advance_width_of(gi) + self.kerning_between(last, gi);
            (width, gi)
        }).0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[derive(Copy, Clone, Eq, PartialEq)]
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