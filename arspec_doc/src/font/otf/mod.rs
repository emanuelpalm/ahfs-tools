#![allow(dead_code)]

mod cmap;
mod glyf;
mod head;
mod hhea;
mod hmtx;
mod kern;
mod loca;
mod maxp;
mod region;

pub use self::cmap::CharacterToGlyphIndexMappingTable;
pub use self::glyf::GlyphDataTable;
pub use self::head::FontHeaderTable;
pub use self::hhea::HorizontalHeaderTable;
pub use self::hmtx::{HorizontalMetrics, HorizontalMetricsTable};
pub use self::kern::KerningTable;
pub use self::maxp::MaximumProfileTable;

use self::loca::IndexToLocationTable;
use self::region::Region;

/// An OpenType Font (OTF) file.
///
/// Provides access to some of the tables in an OTF file. The implementation is
/// designed only to support reading the fonts that come bundled with the
/// application.
pub struct FontFile<'a> {
    cmap: CharacterToGlyphIndexMappingTable<'a>,
    glyf: GlyphDataTable<'a>,
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
        let mut glyf = None;
        let mut head = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut loca = None;
        let mut maxp = None;
        {
            let table_count = file.read_u16_at(4)? as usize;
            for i in 0..table_count {
                let offset = 12 + 16 * i;
                let target = match file.get(offset..offset + 4)? {
                    b"cmap" => &mut cmap,
                    b"glyf" => &mut glyf,
                    b"head" => &mut head,
                    b"hhea" => &mut hhea,
                    b"hmtx" => &mut hmtx,
                    b"kern" => &mut kern,
                    b"loca" => &mut loca,
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
        let kern = kern.and_then(|kern| KerningTable::try_new(kern));
        let loca = IndexToLocationTable::try_new(
            loca?,
            head.index_to_loc_format()
        )?;
        let maxp = MaximumProfileTable::try_new(maxp?)?;
        let glyf = GlyphDataTable::new(glyf?, loca);
        let hmtx = HorizontalMetricsTable::try_new(
            maxp.num_glyphs(),
            hhea.number_of_h_metrics(),
            hmtx?,
        )?;

        Some(FontFile {
            cmap,
            glyf,
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

    /// Glyph table.
    #[inline]
    pub fn glyf(&self) -> &GlyphDataTable<'a> {
        &self.glyf
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
