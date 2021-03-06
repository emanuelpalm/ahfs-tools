#![allow(dead_code)]

use std::convert::TryFrom;
use std::u16;
use super::Region;

/// Table containing metrics for horizontal text layout for individual glyphs.
pub struct HorizontalMetricsTable<'a> {
    num_glyphs: u16,
    number_of_h_metrics: u16,
    region: Region<'a>,
}

impl<'a> HorizontalMetricsTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(
        num_glyphs: u16,
        number_of_h_metrics: u16,
        hmtx: Region<'a>,
    ) -> Option<Self>
    {
        if number_of_h_metrics > num_glyphs {
            return None;
        }
        Some(HorizontalMetricsTable {
            num_glyphs,
            number_of_h_metrics,
            region: hmtx
        })
    }

    /// Get horizontal metrics for glyph at given `index`.
    pub fn lookup(&self, glyph_index: u32) -> Option<HorizontalMetrics> {
        let glyph_index = match u16::try_from(glyph_index).ok() {
            Some(glyph_index) => glyph_index,
            None => { return None; },
        };
        let (aw_index, lsb_index) = if glyph_index < self.number_of_h_metrics {
            let offset = 4 * glyph_index;
            (offset, offset + 2)
        } else if glyph_index < self.num_glyphs {
            let offset = 4 * self.number_of_h_metrics;
            (offset - 4, offset + 2 * glyph_index)
        } else {
            return None;
        };
        Some(HorizontalMetrics {
            advance_width: self.region.read_u16_at(aw_index as usize)?,
            lsb: self.region.read_i16_at(lsb_index as usize)?,
        })
    }
}

/// Horizontal metrics of some glyph.
pub struct HorizontalMetrics {
    /// Advance width, in font design units.
    pub advance_width: u16,

    /// Glyph left side bearing, in font design units.
    pub lsb: i16,
}