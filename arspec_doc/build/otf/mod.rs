#![allow(dead_code)]

mod cmap;
mod error;
mod glyf;
mod head;
mod hhea;
mod hmtx;
mod kern;
mod loca;
mod maxp;
mod region;

pub use self::cmap::CharacterToGlyphIndexMappingTable;
pub use self::error::Error;
pub use self::glyf::GlyphDataTable;
pub use self::head::FontHeaderTable;
pub use self::hhea::HorizontalHeaderTable;
pub use self::hmtx::{HorizontalMetrics, HorizontalMetricsTable};
pub use self::kern::KerningTable;
pub use self::maxp::MaximumProfileTable;

use self::loca::IndexToLocationTable;
use self::region::Region;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// An OpenType Font (OTF) file.
///
/// Provides access to some of the tables in an OTF file. The implementation is
/// only guaranteed to support reading the fonts that come bundled with this
/// package.
pub struct FontFile<'a> {
    file: Region<'a>,
    cmap: CharacterToGlyphIndexMappingTable<'a>,
    glyf: GlyphDataTable<'a>,
    head: FontHeaderTable<'a>,
    hhea: HorizontalHeaderTable<'a>,
    hmtx: HorizontalMetricsTable<'a>,
    kern: Option<KerningTable<'a>>,
    maxp: MaximumProfileTable<'a>,
}

impl<'a> FontFile<'a> {
    #[doc(hidden)]
    pub fn try_new(file: &'a [u8]) -> Result<Self> {
        let file = Region::new(file);

        // Check file version.
        if file.read_u32_at(0).ok_or(Error::SFNT)? != 0x00010000 {
            return Err(Error::SFNT);
        }

        // Load table regions.
        let mut cmap = None;
        let mut glyf = None;
        let mut head = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut loca = None;
        let mut maxp = None;
        {
            let table_count = file.read_u16_at(4).ok_or(Error::SFNT)? as usize;
            for i in 0..table_count {
                let offset = 12 + 16 * i;
                let target = match file.get(offset..offset + 4).ok_or(Error::SFNT)? {
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
                let from = file.read_u32_at(offset + 8).ok_or(Error::SFNT)? as usize;
                let to = from + file.read_u32_at(offset + 12).ok_or(Error::SFNT)? as usize;
                *target = file.subregion(from..to);
            }
        }
        let cmap = cmap.ok_or(Error::SFNT)?;
        let glyf = glyf.ok_or(Error::SFNT)?;
        let head = head.ok_or(Error::SFNT)?;
        let hhea = hhea.ok_or(Error::SFNT)?;
        let hmtx = hmtx.ok_or(Error::SFNT)?;
        let loca = loca.ok_or(Error::SFNT)?;
        let maxp = maxp.ok_or(Error::SFNT)?;

        // Initialize tables.
        let cmap = CharacterToGlyphIndexMappingTable::try_new(&file, cmap).ok_or(Error::CMAP)?;
        let head = FontHeaderTable::try_new(head).ok_or(Error::HEAD)?;
        let hhea = HorizontalHeaderTable::try_new(hhea).ok_or(Error::HHEA)?;
        let kern = kern.and_then(|kern| KerningTable::try_new(kern));
        let loca = IndexToLocationTable::try_new(loca, head.index_to_loc_format())
            .ok_or(Error::LOCA)?;
        let maxp = MaximumProfileTable::try_new(maxp).ok_or(Error::MAXP)?;
        let glyf = GlyphDataTable::new(glyf, loca);
        let hmtx = HorizontalMetricsTable::try_new(
            maxp.num_glyphs(),
            hhea.number_of_h_metrics(),
            hmtx,
        ).ok_or(Error::HMTX)?;

        Ok(FontFile {
            file,
            cmap,
            glyf,
            head,
            hhea,
            hmtx,
            kern,
            maxp,
        })
    }

    /// Get font file as slice of bytes.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.file.bytes()
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
