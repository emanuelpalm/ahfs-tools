mod otf;

pub struct Font<'a> {
    file: otf::FontFile<'a>,
}

impl<'a> Font<'a> {
    pub fn advance_width_of(&self, ch: char) -> f32 {
        let glyph_index = self.file.cmap().lookup(ch);
        (match self.file.hmtx().lookup(glyph_index) {
            Some(metrics) => metrics.advance_width,
            None => self.file.hhea().advance_width_max(),
        }) as f32 / self.file.head().units_per_em() as f32
    }

    pub fn kerning_between(&self, a: char, b: char) -> f32 {
        let kern = match self.file.kern() {
            Some(kern) => kern,
            None => { return 0.0; }
        };
        let cmap = self.file.cmap();
        let a = cmap.lookup(a);
        let b = cmap.lookup(b);
        kern.lookup(a, b) as f32 / self.file.head().units_per_em() as f32
    }

    pub fn line_height(&self) -> f32 {
        let hhea = self.file.hhea();
        let line_height = hhea.ascender() - hhea.descender() + hhea.line_gap();
        line_height as f32 / self.file.head().units_per_em() as f32
    }
}