mod cmap;
mod hhea;
mod hmtx;
mod kern;
mod maxp;
mod region;

pub use self::cmap::CharacterToGlyphIndexMappingTable;
pub use self::hhea::HorizontalHeaderTable;
pub use self::hmtx::{HorizontalMetrics, HorizontalMetricsTable};
pub use self::kern::KerningTable;
pub use self::maxp::MaximumProfileTable;

use self::region::Region;

/// An OpenType font file.
pub struct FontFile<'a> {
    file: Region<'a>,

    cmap: CharacterToGlyphIndexMappingTable<'a>,
    hhea: HorizontalHeaderTable<'a>,
    hmtx: HorizontalMetricsTable<'a>,
    kern: Option<KerningTable<'a>>,
    maxp: MaximumProfileTable<'a>,
}

impl<'a> FontFile<'a> {
    pub fn try_from(file: &'a [u8]) -> Option<Self> {
        // TODO: Fine-grained error detection.

        let file = Region::new(file);

        let mut cmap = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut maxp = None;
        {
            let table_count = file.read_u16_at(4)? as usize;
            for i in 0..table_count {
                let offset = 12 + 16 * i;
                let target = match file.get(offset..offset + 4)? {
                    b"cmap" => &mut cmap,
                    b"hhea" => &mut hhea,
                    b"hmtx" => &mut hmtx,
                    b"kern" => &mut kern,
                    b"maxp" => &mut maxp,
                    _ => continue,
                };
                let from = file.read_u32_at(offset + 8)? as usize;
                let to = from + file.read_u32_at(offset + 12)? as usize;
                *target = file.subsection(from..to);
            }
        }

        let cmap = CharacterToGlyphIndexMappingTable::try_from(&file, cmap?)?;
        let hhea = HorizontalHeaderTable::try_from(hhea?)?;
        let maxp = MaximumProfileTable::try_from(maxp?)?;
        let kern = kern.and_then(|kern| KerningTable::try_from(kern));

        let hmtx = HorizontalMetricsTable::try_from(
            maxp.num_glyphs(),
            hhea.number_of_h_metrics(),
            hmtx?,
        )?;

        Some(FontFile {
            file,
            cmap,
            hhea,
            hmtx,
            kern,
            maxp,
        })
    }

    /// Character to glyph mapping.
    #[inline]
    pub fn cmap(&self) -> &CharacterToGlyphIndexMappingTable<'a> {
        &self.cmap
    }

    /// Horizontal header.
    #[inline]
    pub fn hhea(&self) -> &HorizontalHeaderTable<'a> {
        &self.hhea
    }

    /// Horizontal metrics.
    #[inline]
    pub fn hmtx(&self) -> &HorizontalMetricsTable<'a> {
        &self.hmtx
    }

    /// Kerning.
    #[inline]
    pub fn kern(&self) -> &Option<KerningTable<'a>> {
        &self.kern
    }

    /// Maximum profile.
    #[inline]
    pub fn maxp(&self) -> &MaximumProfileTable<'a> {
        &self.maxp
    }
}
