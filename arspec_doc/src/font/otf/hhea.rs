#![allow(dead_code)]

use super::Region;

/// Table containing metrics for horizontal text layout shared by all glyphs.
pub struct HorizontalHeaderTable<'a> {
    region: Region<'a>,
}

impl<'a> HorizontalHeaderTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(hhea: Region<'a>) -> Option<Self> {
        let major_version = hhea.read_u16_at(0)?;
        if major_version < 1 {
            return None;
        }
        if hhea.len() < 36 {
            return None;
        }
        Some (HorizontalHeaderTable { region: hhea })
    }

    /// Typographic ascent (Distance from the baseline of highest ascender).
    #[inline]
    pub fn ascender(&self) -> i16 {
        self.region.read_i16_at(4).unwrap_or(0)
    }

    /// Typographic descent (Distance from baseline of lowest descender).
    #[inline]
    pub fn descender(&self) -> i16 {
        self.region.read_i16_at(6).unwrap_or(0)
    }

    /// Typographic line gap.
    #[inline]
    pub fn line_gap(&self) -> i16 {
        self.region.read_i16_at(8).unwrap_or(0)
    }

    /// Maximum advance width value in 'hmtx' table.
    #[inline]
    pub fn advance_width_max(&self) -> u16 {
        self.region.read_u16_at(10).unwrap_or(0)
    }

    /// Minimum left sidebearing value in 'hmtx' table.
    #[inline]
    pub fn min_lsb(&self) -> i16 {
        self.region.read_i16_at(12).unwrap_or(0)
    }

    /// Minimum right sidebearing value; calculated as
    /// Min(aw - lsb - (xMax - xMin)).
    #[inline]
    pub fn min_rsb(&self) -> i16 {
        self.region.read_i16_at(14).unwrap_or(0)
    }

    /// Max(lsb + (xMax - xMin)).
    #[inline]
    pub fn x_max_extent(&self) -> i16 {
        self.region.read_i16_at(16).unwrap_or(0)
    }

    /// Used to calculate the slope of the cursor (rise/run); 1 for vertical.
    #[inline]
    pub fn caret_slope_rise(&self) -> i16 {
        self.region.read_i16_at(18).unwrap_or(0)
    }

    /// 0 for vertical.
    #[inline]
    pub fn caret_slope_run(&self) -> i16 {
        self.region.read_i16_at(20).unwrap_or(0)
    }

    /// The amount by which a slanted highlight on a glyph needs to be shifted
    /// to produce the best appearance. Set to 0 for non-slanted fonts.
    #[inline]
    pub fn caret_offset(&self) -> i16 {
        self.region.read_i16_at(22).unwrap_or(0)
    }

    /// Number of hMetric entries in 'hmtx' table.
    #[inline]
    pub fn number_of_h_metrics(&self) -> u16 {
        self.region.read_u16_at(34).unwrap_or(0)
    }
}