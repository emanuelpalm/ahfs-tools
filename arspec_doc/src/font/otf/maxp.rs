#![allow(dead_code)]

use super::Region;

/// This table establishes the memory requirements for this font.
pub struct MaximumProfileTable<'a> {
    region: Region<'a>,
}

impl<'a> MaximumProfileTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(maxp: Region<'a>) -> Option<Self> {
        let version = maxp.read_u32_at(0)?;
        match version {
            0x00005000 => if maxp.len() < 6 {
                return None;
            },
            0x00010000 => if maxp.len() < 32 {
                return None;
            },
            _ => {
                return None;
            }
        }
        Some(MaximumProfileTable { region: maxp })
    }

    /// Profile table version, which must be either `0x00005000` or
    /// `0x00010000`.
    #[inline]
    pub fn version(&self) -> u32 {
        self.region.read_u32_at(0).unwrap_or(0)
    }

    /// The number of glyphs in the font.
    #[inline]
    pub fn num_glyphs(&self) -> u16 {
        self.region.read_u16_at(4).unwrap_or(0)
    }

    /// Maximum points in a non-composite glyph.
    #[inline]
    pub fn max_points(&self) -> u16 {
        self.region.read_u16_at(6).unwrap_or(0)
    }

    /// Maximum contours in a non-composite glyph.
    #[inline]
    pub fn max_contours(&self) -> u16 {
        self.region.read_u16_at(8).unwrap_or(0)
    }

    /// Maximum points in a composite glyph.
    #[inline]
    pub fn max_composite_points(&self) -> u16 {
        self.region.read_u16_at(10).unwrap_or(0)
    }

    /// Maximum contours in a composite glyph.
    #[inline]
    pub fn max_composite_contours(&self) -> u16 {
        self.region.read_u16_at(12).unwrap_or(0)
    }

    /// 1 if instructions do not use the twilight zone (Z0), or 2 if
    /// instructions do use Z0; should be set to 2 in most cases.
    #[inline]
    pub fn max_zones(&self) -> u16 {
        self.region.read_u16_at(14).unwrap_or(0)
    }

    /// Maximum points used in Z0.
    #[inline]
    pub fn max_twilight_points(&self) -> u16 {
        self.region.read_u16_at(16).unwrap_or(0)
    }

    /// Number of Storage Area locations.
    #[inline]
    pub fn max_storage(&self) -> u16 {
        self.region.read_u16_at(18).unwrap_or(0)
    }

    /// Number of FDEFs, equal to the highest function number + 1.
    #[inline]
    pub fn max_function_defs(&self) -> u16 {
        self.region.read_u16_at(20).unwrap_or(0)
    }

    /// Number of IDEFs.
    #[inline]
    pub fn max_instruction_defs(&self) -> u16 {
        self.region.read_u16_at(22).unwrap_or(0)
    }

    /// Maximum stack depth across Font Program ('fpgm' table), CVT Program
    /// ('prep' table) and all glyph instructions ( in the 'glyf' table).
    #[inline]
    pub fn max_stack_elements(&self) -> u16 {
        self.region.read_u16_at(24).unwrap_or(0)
    }

    /// Maximum byte count for glyph instructions.
    #[inline]
    pub fn max_size_of_instructions(&self) -> u16 {
        self.region.read_u16_at(26).unwrap_or(0)
    }

    /// Maximum number of components referenced at "top level" for any composite
    /// glyph.
    #[inline]
    pub fn max_component_elements(&self) -> u16 {
        self.region.read_u16_at(28).unwrap_or(0)
    }

    /// Maximum levels of recursion; 1 for simple components.
    #[inline]
    pub fn max_component_depth(&self) -> u16 {
        self.region.read_u16_at(30).unwrap_or(0)
    }
}