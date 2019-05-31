#![allow(dead_code)]

use super::Region;

/// If set to 1, the table has horizontal data, 0 if vertical.
const COVERAGE_MASK_HORIZONTAL: u16 = 0b0000_0000_0000_0001;

/// If this bit is set to 1, the table has minimum values. If set to 0, the
/// table has kerning values.
const COVERAGE_MASK_MINIMUM: u16 = 0b0000_0000_0000_0010;

/// If set to 1, kerning is perpendicular to the flow of the text.
///
/// If the text is normally written horizontally, kerning will be done in the
/// up and down directions. If kerning values are positive, the text will be
/// kerned upwards; if they are negative, the text will be kerned downwards.
///
/// If the text is normally written vertically, kerning will be done in the
/// left and right directions. If kerning values are positive, the text will be
/// kerned to the right; if they are negative, the text will be kerned to the
/// left.
///
/// The value 0x8000 in the kerning data resets the cross-stream kerning back
/// to 0.
const COVERAGE_MASK_CROSS_STREAM: u16 = 0b0000_0000_0000_0100;

/// If this bit is set to 1 the value in this table should replace the value
/// currently being accumulated.
const COVERAGE_MASK_OVERRIDE: u16 = 0b0000_0000_0000_1000;

/// Reserved. This should be set to zero.
const COVERAGE_MASK_RESERVED: u16 = 0b0000_0000_1111_0000;

/// Format of the subtable. Only formats 0 and 2 have been defined. Formats 1
/// and 3 through 255 are reserved for future use.
const COVERAGE_MASK_FORMAT: u16 = 0b1111_1111_0000_0000;

/// The kerning table contains the values that control the inter-character
/// spacing for the glyphs in a font.
pub struct KerningTable<'a> {
    subtable: Region<'a>,
}

impl<'a> KerningTable<'a> {
    pub fn try_new(kern: Region<'a>) -> Option<Self> {
        let version = kern.read_u16_at(0)?;
        if version != 0 {
            return None;
        }
        let n_tables = kern.read_u16_at(2)? as usize;

        let mut subtable = None;
        let mut offset = 4;
        for _ in 0..n_tables {
            let length = kern.read_u16_at(offset + 2)?;
            let coverage = kern.read_u16_at(offset + 4)?;
            if coverage == COVERAGE_MASK_HORIZONTAL {
                subtable = kern.subregion(offset..offset + length as usize);
                break;
            }
            offset += length as usize;
        }
        Some(KerningTable { subtable: subtable? })
    }

    /// Looks up the space, in font design units, between glyphs `a` and `b`.
    ///
    /// The order of `a` and `b` is significant.
    pub fn lookup(&self, a: u32, b: u32) -> i16 {
        if a > u16::max_value() as u32 || b > u16::max_value() as u32 {
            return 0;
        }
        let mut left: i32 = 0;
        let mut right: i32 = match self.subtable.read_u16_at(6) {
            Some(x) => x as i32 - 1,
            None => { return 0; }
        };
        let ab = a << 16 | b;
        while left <= right {
            let middle = (left + right) / 2;
            let offset = 14 + (middle as usize) * 6;
            let ab0 = match self.subtable.read_u32_at(offset) {
                Some(x) => x,
                None => { break; }
            };
            if ab < ab0 {
                right = middle - 1;
            } else if ab > ab0 {
                left = middle + 1;
            } else {
                return self.subtable.read_i16_at(offset + 4).unwrap_or(0);
            }
        }
        0
    }
}