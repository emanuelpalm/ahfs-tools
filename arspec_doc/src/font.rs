static FONT_MONO: Font<'static> = include!(concat!(env!("OUT_DIR"), "/font_mono.rs"));
static FONT_SANS: Font<'static> = include!(concat!(env!("OUT_DIR"), "/font_sans.rs"));
static FONT_SANS_BOLD: Font<'static> = include!(concat!(env!("OUT_DIR"), "/font_sans_bold.rs"));
static FONT_SANS_ITALIC: Font<'static> = include!(concat!(env!("OUT_DIR"), "/font_sans_italic.rs"));

pub type GlyphIndex = u16;

pub struct Font<'a> {
    line_height: i16,
    units_per_em: u16,

    glyph_indexes_nonascii: &'a [u16],
    advance_widths: &'a [u16],
    kernings: Option<&'a [&'a [(GlyphIndex, i16)]]>,
}

impl<'a> Font<'a> {
    #[inline]
    pub fn mono() -> &'static Font<'static> {
        &FONT_MONO
    }

    #[inline]
    pub fn sans() -> &'static Font<'static> {
        &FONT_SANS
    }

    #[inline]
    pub fn sans_bold() -> &'static Font<'static> {
        &FONT_SANS_BOLD
    }

    #[inline]
    pub fn sans_italic() -> &'static Font<'static> {
        &FONT_SANS_ITALIC
    }

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

    #[inline]
    pub fn advance_width_of(&self, gi: GlyphIndex) -> u16 {
        self.advance_widths[gi as usize]
    }

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

    #[inline]
    pub fn line_height(&self) -> i16 {
        self.line_height
    }

    #[inline]
    pub fn line_width_of(&self, line: &str) -> i32 {
        line.chars().fold((0, 0), |(mut width, last), ch| {
            let gi = self.glyph_index_of(ch);
            width += self.advance_width_of(gi) as i32 + self.kerning_between(last, gi) as i32;
            (width, gi)
        }).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_width_benchmark() {
        let line = concat!(
            "Töja!",
        );

        let mut font = Font::sans();
        let mut width = 0;
        for _ in 0..1 {
            width = font.line_width_of(line);
        }
        panic!("{}", width)
    }
}
