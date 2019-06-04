#![allow(dead_code)]

use std::ops;
use super::Region;

/// This table stores offsets to the locations of the glyphs in the font,
/// relative to the beginning of the `glyf` table.
pub struct IndexToLocation<'a> {
    region: Region<'a>,
    format: Format,
}

impl<'a> IndexToLocation<'a> {
    pub fn try_new(loca: Region<'a>, format: i16) -> Option<Self> {
        Some(IndexToLocation {
            region: loca,
            format: Format::try_new(format)?,
        })
    }

    /// Looks up range of bytes used by identified glyph in the `glyf` table.
    pub fn lookup(&self, glyph_index: u32) -> Option<ops::Range<usize>> {
        match self.format {
            Format::Short => {
                self.region.read_2x_u16_at(glyph_index as usize * 2)
                    .map(|(x1, x2)| (x1 as usize * 2)..(x2 as usize * 2))
            }
            Format::Long => {
                self.region.read_2x_u32_at(glyph_index as usize * 4)
                    .map(|(x1, x2)| (x1 as usize)..(x2 as usize))
            }
        }
    }
}

enum Format {
    Short,
    Long,
}

impl Format {
    #[inline]
    pub fn try_new(format: i16) -> Option<Format> {
        Some(match format {
            0 => Format::Short,
            1 => Format::Long,
            _ => { return None; }
        })
    }
}