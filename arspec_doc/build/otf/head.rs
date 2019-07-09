#![allow(dead_code)]

use super::Region;

const MAC_STYLE_FLAG_BOLD: u16 = (1 << 0);
const MAC_STYLE_FLAG_ITALIC: u16 = (1 << 1);
const MAC_STYLE_FLAG_UNDERLINE: u16 = (1 << 2);
const MAC_STYLE_FLAG_OUTLINE: u16 = (1 << 3);
const MAC_STYLE_FLAG_SHADOW: u16 = (1 << 4);
const MAC_STYLE_FLAG_CONDENSED: u16 = (1 << 5);
const MAC_STYLE_FLAG_EXTENDED: u16 = (1 << 6);

pub struct FontHeaderTable<'a> {
    region: Region<'a>
}

impl<'a> FontHeaderTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(head: Region<'a>) -> Option<Self> {
        let major_version = head.read_u16_at(0)?;
        if major_version != 1 {
            return None;
        }

        let minor_version = head.read_u16_at(2)?;
        if minor_version != 0 {
            return None;
        }

        let magic_number = head.read_u32_at(12)?;
        if magic_number != 0x5F0F3CF5 {
            return None;
        }

        if head.len() < 54 {
            return None;
        }

        Some(FontHeaderTable {
            region: head,
        })
    }

    /// Major version number of the font header table — set to 1.
    #[inline]
    pub fn major_version(&self) -> u16 {
        self.region.read_u16_at(0).unwrap_or(0)
    }

    /// Minor version number of the font header table — set to 0.
    #[inline]
    pub fn minor_version(&self) -> u16 {
        self.region.read_u16_at(2).unwrap_or(0)
    }

    /// Set by font manufacturer.
    #[inline]
    pub fn font_revision(&self) -> u32 {
        self.region.read_u32_at(4).unwrap_or(0)
    }

    /// To compute: set it to 0, sum the entire font as `u32`, then store
    /// `0xB1B0AFBA - sum`. If the font is used as a component in a font
    /// collection file, the value of this field will be invalidated by changes
    /// to the file structure and font table directory, and must be ignored.
    #[inline]
    pub fn check_sum_adjustment(&self) -> u32 {
        self.region.read_u32_at(8).unwrap_or(0)
    }

    /// Set to 0x5F0F3CF5.
    #[inline]
    pub fn magic_number(&self) -> u32 {
        self.region.read_u32_at(12).unwrap_or(0)
    }

    /// Bit flags.
    ///
    /// - Bit 0: Baseline for font at y=0;.
    /// - Bit 1: Left sidebearing point at x=0 (relevant only for TrueType
    ///   rasterizers) — see the note below regarding variable fonts.
    /// - Bit 2: Instructions may depend on point size.
    /// - Bit 3: Force ppem to integer values for all internal scaler math; may
    ///   use fractional ppem sizes if this bit is clear.
    /// - Bit 4: Instructions may alter advance width (the advance widths might
    ///   not scale linearly).
    /// - Bit 5: This bit is not used in OpenType, and should not be set in
    ///   order to ensure compatible behavior on all platforms. If set, it may
    ///   result in different behavior for vertical layout in some platforms.
    ///   (See Apple’s specification for details regarding behavior in Apple
    ///   platforms.)
    /// - Bits 6–10: These bits are not used in Opentype and should always be
    ///   cleared. (See Apple’s specification for details regarding legacy used
    ///   in Apple platforms.)
    /// - Bit 11: Font data is “lossless” as a result of having been subjected
    ///   to optimizing transformation and/or compression (such as e.g.
    ///   compression mechanisms defined by ISO/IEC 14496-18, MicroType
    ///   Express, WOFF 2.0 or similar) where the original font functionality
    ///   and features are retained but the binary compatibility between input
    ///   and output font files is not guaranteed. As a result of the applied
    ///   transform, the DSIG table may also be invalidated.
    /// - Bit 12: Font converted (produce compatible metrics).
    /// - Bit 13: Font optimized for ClearType™. Note, fonts that rely on
    ///   embedded bitmaps (EBDT) for rendering should not be considered
    ///   optimized for ClearType, and therefore should keep this bit cleared.
    /// - Bit 14: Last Resort font. If set, indicates that the glyphs encoded
    ///   in the 'cmap' subtables are simply generic symbolic representations
    ///   of code point ranges and don’t truly represent support for those code
    ///   points. If unset, indicates that the glyphs encoded in the `cmap`
    ///   subtables represent proper support for those code points.
    /// - Bit 15: Reserved, set to 0.
    #[inline]
    pub fn flags(&self) -> u16 {
        self.region.read_u16_at(16).unwrap_or(0)
    }

    /// Set to a value from 16 to 16384. Any value in this range is valid. In
    /// fonts that have TrueType outlines, a power of 2 is recommended as this
    /// allows performance optimizations in some rasterizers.
    #[inline]
    pub fn units_per_em(&self) -> u16 {
        self.region.read_u16_at(18).unwrap_or(0)
    }

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in
    /// GMT/UTC time zone.
    #[inline]
    pub fn created(&self) -> i64 {
        self.region.read_i64_at(20).unwrap_or(0)
    }

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in
    /// GMT/UTC time zon
    #[inline]
    pub fn modified(&self) -> i64 {
        self.region.read_i64_at(28).unwrap_or(0)
    }

    /// For all glyph bounding boxes.
    #[inline]
    pub fn x_min(&self) -> u16 {
        self.region.read_u16_at(36).unwrap_or(0)
    }

    /// For all glyph bounding boxes.
    #[inline]
    pub fn y_min(&self) -> u16 {
        self.region.read_u16_at(38).unwrap_or(0)
    }

    /// For all glyph bounding boxes.
    #[inline]
    pub fn x_max(&self) -> u16 {
        self.region.read_u16_at(40).unwrap_or(0)
    }

    /// For all glyph bounding boxes.
    #[inline]
    pub fn y_max(&self) -> u16 {
        self.region.read_u16_at(42).unwrap_or(0)
    }

    /// Mac style bit flags.
    ///
    /// - Bit 0: Bold (if set to 1).
    /// - Bit 1: Italic (if set to 1).
    /// - Bit 2: Underline (if set to 1).
    /// - Bit 3: Outline (if set to 1).
    /// - Bit 4: Shadow (if set to 1).
    /// - Bit 5: Condensed (if set to 1).
    /// - Bit 6: Extended (if set to 1).
    /// - Bits 7–15: Reserved (set to 0).
    #[inline]
    pub fn mac_style(&self) -> u16 {
        self.region.read_u16_at(44).unwrap_or(0)
    }

    /// Whether Mac style BOLD flag is set.
    #[inline]
    pub fn mac_style_bold(&self) -> bool {
        self.mac_style() & MAC_STYLE_FLAG_BOLD != 0
    }

    /// Whether Mac style ITALIC flag is set.
    #[inline]
    pub fn mac_style_italic(&self) -> bool {
        self.mac_style() & MAC_STYLE_FLAG_ITALIC != 0
    }

    /// Smallest readable size in pixels.
    #[inline]
    pub fn lowest_rec_ppem(&self) -> u16 {
        self.region.read_u16_at(46).unwrap_or(0)
    }

    /// 0 for short offsets (Offset16), 1 for long (Offset32).
    #[inline]
    pub fn index_to_loc_format(&self) -> i16 {
        self.region.read_i16_at(50).unwrap_or(0)
    }

    /// 0 for current format.
    #[inline]
    pub fn glyph_data_format(&self) -> i16 {
        self.region.read_i16_at(52).unwrap_or(0)
    }
}
