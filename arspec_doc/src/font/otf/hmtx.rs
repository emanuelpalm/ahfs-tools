use super::Region;

/// Table containing metrics for horizontal text layout for individual glyphs.
pub struct HorizontalMetricsTable<'a> {
    num_glyphs: u16,
    number_of_h_metrics: u16,
    region: Region<'a>,
}

impl<'a> HorizontalMetricsTable<'a> {
    #[inline]
    pub fn try_from(
        num_glyphs: u16,
        number_of_h_metrics: u16,
        region: Region<'a>,
    ) -> Option<Self>
    {
        if number_of_h_metrics > num_glyphs {
            return None;
        }
        Some(HorizontalMetricsTable { num_glyphs, number_of_h_metrics, region })
    }

    /// Get horizontal metrics for glyph at given `index`.
    pub fn get(&self, index: u16) -> Option<HorizontalMetrics> {
        let (aw_index, lsb_index) = if index < self.number_of_h_metrics {
            let offset = 4 * index;
            (index, index + 2)
        } else if index < self.num_glyphs {
            let offset = 4 * self.number_of_h_metrics;
            (offset - 4, offset + 2 * index)
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