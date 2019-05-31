#![allow(dead_code)]

mod cmap;
mod head;
mod hhea;
mod hmtx;
mod kern;
mod maxp;
mod region;

pub use self::cmap::CharacterToGlyphIndexMappingTable;
pub use self::head::FontHeaderTable;
pub use self::hhea::HorizontalHeaderTable;
pub use self::hmtx::{HorizontalMetrics, HorizontalMetricsTable};
pub use self::kern::KerningTable;
pub use self::maxp::MaximumProfileTable;

use self::region::Region;

/// An OpenType font file.
///
/// Currently, this implementation is designed only to support metrics
/// calculations for European fonts. In particular, it is only strictly
/// required to be able to read the fonts that comes bundled with this
/// application.
pub struct FontFile<'a> {
    cmap: CharacterToGlyphIndexMappingTable<'a>,
    head: FontHeaderTable<'a>,
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
        let mut head = None;
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
                    b"head" => &mut head,
                    b"hhea" => &mut hhea,
                    b"hmtx" => &mut hmtx,
                    b"kern" => &mut kern,
                    b"maxp" => &mut maxp,
                    _ => continue,
                };
                let from = file.read_u32_at(offset + 8)? as usize;
                let to = from + file.read_u32_at(offset + 12)? as usize;
                *target = file.subregion(from..to);
            }
        }

        let cmap = CharacterToGlyphIndexMappingTable::try_new(&file, cmap?)?;
        let head = FontHeaderTable::try_new(head?)?;
        let hhea = HorizontalHeaderTable::try_new(hhea?)?;
        let maxp = MaximumProfileTable::try_new(maxp?)?;
        let kern = kern.and_then(|kern| KerningTable::try_new(kern));

        let hmtx = HorizontalMetricsTable::try_new(
            maxp.num_glyphs(),
            hhea.number_of_h_metrics(),
            hmtx?,
        )?;

        Some(FontFile {
            cmap,
            head,
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

    /// Font header.
    #[inline]
    pub fn head(&self) -> &FontHeaderTable<'a> {
        &self.head
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
    pub fn kern(&self) -> Option<&KerningTable<'a>> {
        self.kern.as_ref()
    }

    /// Maximum profile.
    #[inline]
    pub fn maxp(&self) -> &MaximumProfileTable<'a> {
        &self.maxp
    }
}
