pub struct Font<'a> {
    advance_widths: &'a [u16],
    advance_width_index: &'a[u16],

    kernings: &'a [i16],
    kerning_index: &'a [u32],

    line_height: u16,
    units_per_em: u16,
}

impl<'a> Font<'a> {
    /*
    fn try_from(file: &'a [u8]) -> Result<Self> {
        let file = otf::FontFile::try_from(file)?;
        let advance_width_max = file.hhea().advance_width_max() as f32;
        let units_per_em = file.head().units_per_em() as f32;
        let line_height = {
            let hhea = file.hhea();
            (hhea.ascender() - hhea.descender() + hhea.line_gap()) as f32 / units_per_em
        };
        Ok(Font {
            file,
            advance_width_max,
            line_height,
            units_per_em,
            advance_width_cache: RefCell::new([u16::min_value(); 127]),
            glyph_index_cache: RefCell::new([usize::min_value(); 127]),
        })
    }*/

    pub fn advance_width_of(&self, ch: char) -> f32 {
        if ch as u32 > u16::max_value() as u32 {
            return 0.0;
        }
        let target = ch as u32 as u16;
        self.advance_width_index.binary_search(&target)
            .map(|index| self.advance_widths[index])
            .unwrap_or(0) as f32 / (self.units_per_em as f32)
    }
    
    pub fn kerning_between(&self, a: char, b: char) -> f32 {
        if a as u32 > u16::max_value() as u32 {
            return 0.0;
        }
        if b as u32 > u16::max_value() as u32 {
            return 0.0;
        }
        let target = (a as u32) | ((b as u32) << 16);
        self.kerning_index.binary_search(&target)
            .map(|index| self.kernings[index])
            .unwrap_or(0) as f32 / (self.units_per_em as f32)
    }

    #[inline]
    pub fn line_height(&self) -> f32 {
        self.line_height as f32
    }

    pub fn line_width_of(&self, line: &str) -> f32 {
        // TODO: Seems to yield slightly incorrect result. Rounding issue?
        line.chars().fold((0.0, '\0'), |(mut width, last), ch| {
            width += self.advance_width_of(ch) + self.kerning_between(last, ch);
            (width, ch)
        }).0
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_width_benchmark() {
        let line = concat!(
            "Napriek všeobecnému presvedčeniu nie je Lorem Ipsum len náhodný ",
            "text. Jeho korene sú v časti klasickej latinskej literatúry z ",
            "roku 45 pred n.l., takže má viac ako 2000 rokov. Richard ",
            "McClintock, profesor latinčiny na Hampden-Sydney College vo ",
            "Virgínii, hľadal jedno z menej určitých latinských slov, ",
            "consectetur, z pasáže Lorem Ipsum, a ako vyhľadával výskyt tohto ",
            "slova v klasickej literatúre, objavil jeho nepochybný zdroj. ",
            "Lorem Ipsum pochádza z odsekov 1.10.32 a 1.10.33 Cicerovho diela ",
            "'De finibus bonorum et malorum' (O najvyššom dobre a zle), ",
            "napísaného v roku 45 pred n.l. Táto kniha je pojednaním o teórii ",
            "etiky, a bola veľmi populárna v renesancii. Prvý riadok Lorem ",
            "Ipsum, 'Lorem ipsum dolor sit amet..', je z riadku v odseku ",
            "1.10.32. Štandardný úsek Lorem Ipsum, používaný od 16. storočia, ",
            "je pre zaujímavosť uvedený nižšie. Odseky 1.10.32 a 1.10.33 z ",
            "'De finibus bonorum et malorum' od Cicera tu sú tiež uvedené v ",
            "ich presnom pôvodnom tvare, doplnené anglickými verziami z roku ",
            "1914, v preklade H. Rackhama.",
        );

        let mut font = Font::load_sans().expect("Failed to load font.");
        let mut width = 0.0;
        for _ in 0..999 {
            width = font.line_width_of(line);
        }
        panic!("{}", width)
    }
}
*/