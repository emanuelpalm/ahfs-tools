mod error;
mod otf;

use self::error::Error;
use std::cell::RefCell;
use std::result;

const FONT_MONO: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoMono-Regular-pruned.ttf"));
const FONT_SANS: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Regular-pruned.ttf"));
const FONT_SANS_BOLD: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Bold-pruned.ttf"));
const FONT_SANS_ITALIC: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Italic-pruned.ttf"));

pub type Result<T> = result::Result<T, Error>;

pub struct Font<'a> {
    file: otf::FontFile<'a>,
    advance_width_max: f32,
    line_height: f32,
    units_per_em: f32,
    advance_width_cache: RefCell<[u16; 127]>, // TODO: Load on init rather than lazily. Shrink.
    glyph_index_cache: RefCell<[usize; 127]>, // TODO: Load on init rather than lazily. Shrink.
}

macro_rules! load_font {
    (pub fn $name:ident (); $font:expr; $doc:expr) => {
        #[doc = $doc]
        #[inline]
        pub fn $name() -> Result<Self> {
            Self::try_from($font)
        }
    };
}

impl<'a> Font<'a> {
    load_font!(
        pub fn load_mono(); FONT_MONO;
        "Default monospaced font."
    );
    load_font!(
        pub fn load_sans(); FONT_SANS;
        "Default sans-serif font."
    );
    load_font!(
        pub fn load_sans_bold(); FONT_SANS_BOLD;
        "Default sans-serif font, bold variant."
    );
    load_font!(
        pub fn load_sans_italic(); FONT_SANS_ITALIC;
        "Default sans-serif font, italicized variant."
    );

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
    }

    // TODO: Take glyph index as argument instead of char.
    pub fn advance_width_of(&self, ch: char) -> f32 {
        let is_ascii = (ch as u32) <= 127;
        if is_ascii {
            let cache = self.advance_width_cache.borrow();
            let value = cache[ch as u8 as usize];
            if value != u16::min_value() {
                return value as f32 / self.units_per_em;
            }
        }
        let glyph_index = self.glyph_index_of(ch);
        let advance_width = match self.file.hmtx().lookup(glyph_index) {
            Some(metrics) => metrics.advance_width,
            None => { return self.advance_width_max; },
        };
        if is_ascii {
            self.advance_width_cache.borrow_mut()[ch as u32 as usize] = advance_width;
        }
        advance_width as f32 / self.units_per_em
    }

    fn glyph_index_of(&self, ch: char) -> usize {
        let is_ascii = (ch as u32) <= 127;
        if is_ascii {
            let cache = self.glyph_index_cache.borrow();
            let value = cache[ch as u8 as usize];
            if value != usize::min_value() {
                return value;
            }
        }
        let glyph_index = self.file.cmap().lookup(ch);
        if is_ascii {
            self.glyph_index_cache.borrow_mut()[ch as u32 as usize] = glyph_index;
        }
        glyph_index
    }

    // TODO: Take glyph indexes as argument instead of chars.
    pub fn kerning_between(&self, a: char, b: char) -> f32 {
        let kern = match self.file.kern() {
            Some(kern) => kern,
            None => { return 0.0; }
        };
        let a = self.glyph_index_of(a);
        let b = self.glyph_index_of(b);
        kern.lookup(a, b) as f32 / self.units_per_em
    }

    #[inline]
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    pub fn line_width_of(&self, line: &str) -> f32 {
        // TODO: Seems to yield slightly incorrect result. Rounding issue?
        line.chars().fold((0.0, '\0'), |(mut width, last), ch| {
            width += self.advance_width_of(ch) + self.kerning_between(last, ch);
            (width, ch)
        }).0
    }
}

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