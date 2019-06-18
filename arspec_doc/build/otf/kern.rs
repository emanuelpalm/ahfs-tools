#![allow(dead_code)]

use super::Region;

const COVERAGE_MASK_HORIZONTAL: u16 = 0x0001;
const COVERAGE_MASK_MINIMUM: u16 = 0x0002;
const COVERAGE_MASK_CROSS_STREAM: u16 = 0x0004;
const COVERAGE_MASK_OVERRIDE: u16 = 0x0008;
const COVERAGE_MASK_RESERVED: u16 = 0x00F0;
const COVERAGE_MASK_FORMAT: u16 = 0xFF00;

/// The kerning table contains the values that control the inter-character
/// spacing for the glyphs in a font.
pub struct KerningTable<'a> {
    subtable: Region<'a>,
}

impl<'a> KerningTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(kern: Region<'a>) -> Option<Self> {
        let version = kern.read_u16_at(0)?;
        if version != 0 {
            return None;
        }
        let n_tables = kern.read_u16_at(2)? as usize;

        let mut subtable = None;
        let mut offset = 4;
        for _ in 0..n_tables {
            let (length, coverage) = kern.read_2x_u16_at(offset + 2)?;
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
    pub fn lookup(&self, a: usize, b: usize) -> i16 {
        if a > u16::max_value() as usize || b > u16::max_value() as usize {
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
                Some(x) => x as usize,
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