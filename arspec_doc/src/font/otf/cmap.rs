#![allow(dead_code)]

use super::Region;

const PLATFORM_UNICODE: u16 = 0;
const PLATFORM_MACINTOSH: u16 = 1;
const PLATFORM_WINDOWS: u16 = 3;

const PLATFORM_WINDOWS_SYMBOL: u16 = 0;
const PLATFORM_WINDOWS_UNICODE_BMP: u16 = 1;
const PLATFORM_WINDOWS_SHIFT_JIS: u16 = 2;
const PLATFORM_WINDOWS_PRC: u16 = 3;
const PLATFORM_WINDOWS_BIG5: u16 = 4;
const PLATFORM_WINDOWS_WANSUNG: u16 = 5;
const PLATFORM_WINDOWS_JOHAB: u16 = 6;
const PLATFORM_WINDOWS_UNICODE_FULL: u16 = 10;

/// This table defines the mapping of character codes to the glyph index values
/// used in the font.
pub struct CharacterToGlyphIndexMappingTable<'a> {
    subtable: Region<'a>,
    format: Format,
}

impl<'a> CharacterToGlyphIndexMappingTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn try_new(file: &Region<'a>, cmap: Region<'a>) -> Option<Self> {
        let version = cmap.read_u16_at(0)?;
        if version != 0 {
            return None;
        }

        let mut subtable = None;

        let table_count = cmap.read_u16_at(2)? as usize;
        for i in 0..table_count {
            let offset = 4 + 8 * i;
            match cmap.read_u16_at(offset)? {
                PLATFORM_UNICODE => {}
                PLATFORM_WINDOWS => {
                    match cmap.read_u16_at(offset + 2)? {
                        PLATFORM_WINDOWS_UNICODE_BMP |
                        PLATFORM_WINDOWS_UNICODE_FULL => {}
                        _ => continue,
                    }
                }
                _ => continue,
            }
            let index = cmap.read_u32_at(offset + 4)? as usize;
            subtable = file.subregion(cmap.range().start + index..file.range().end);
            break;
        }

        let subtable = subtable?;
        let format = match subtable.read_u16_at(0)? {
            0 => Format::Type0 {
                length: subtable.read_u16_at(2)
                    .map(|len| (len - 6) as usize)
                    .unwrap_or(0),
            },
            4 => Format::Type4 {
                seg_count: subtable.read_u16_at(6).unwrap_or(0) as usize / 2,
                range_shift: subtable.read_u16_at(12).unwrap_or(0) as usize / 2,
            },
            6 => {
                let first = subtable.read_u16_at(6).unwrap_or(0) as usize;
                Format::Type6 {
                    first,
                    end: first + subtable.read_u16_at(8).unwrap_or(0) as usize,
                }
            }
            12 => Format::Type12,
            13 => Format::Type13,
            _ => { return None; }
        };

        Some(CharacterToGlyphIndexMappingTable {
            subtable,
            format,
        })
    }

    /// Acquires glyph index for given char `ch`.
    pub fn lookup(&self, ch: char) -> usize {
        let ch = ch as usize;
        match self.format {
            Format::Type0 { length } => {
                if ch < length {
                    self.subtable.read_u8_at(6 + ch as usize)
                        .unwrap_or(0) as usize
                } else {
                    0
                }
            }
            Format::Type4 { seg_count, range_shift } => {
                if ch > 0xffff {
                    return 0;
                }

                let read_at = |i| self.subtable.read_u16_at(i).unwrap_or(0);

                let mut search_range = (read_at(8) / 2) as usize;
                let mut entry_selector = read_at(10);

                let end_count = 14;
                let mut search = end_count;

                if ch >= read_at(search + range_shift * 2) as usize {
                    search += range_shift * 2;
                }

                search -= 2;
                while entry_selector != 0 {
                    search_range >>= 1;
                    let end = read_at(search + search_range * 2) as usize;
                    if ch > end {
                        search += search_range * 2;
                    }
                    entry_selector -= 1;
                }
                search += 2;

                let item = (search - end_count) / 2;
                if ch > read_at(end_count + 2 * item) as usize {
                    return 0;
                }
                let start = read_at(14 + seg_count * 2 + 2 + 2 * item) as usize;
                if ch < start {
                    return 0;
                }
                let offset = read_at(14 + seg_count * 6 + 2 + 2 * item) as usize;
                if offset == 0 {
                    (ch as i32 + read_at(14 + seg_count * 4 + 2 + 2 * item) as i32)
                        as u16 as usize
                } else {
                    read_at(offset + (ch - start) as usize * 2
                        + 14 + seg_count * 6 + 2 + 2 * item) as usize
                }
            }
            Format::Type6 { first, end } => {
                if ch >= first && ch < end {
                    return self.subtable.read_u16_at(10 + (ch - first) as usize * 2)
                        .unwrap_or(0) as usize;
                }
                0
            }
            format @ Format::Type12 | format @ Format::Type13 => {
                let mut low = 0 as usize;
                let mut high = self.subtable.read_u16_at(12).unwrap_or(0) as usize;

                let read_at = |i| self.subtable.read_u32_at(i)
                    .unwrap_or(0) as usize;

                while low < high {
                    let mid = (low + high) / 2;
                    let group = 16 + (mid * 12) as usize;
                    let start_c = read_at(group);
                    if ch < start_c {
                        high = mid;
                    } else if ch > read_at(group + 4) {
                        low = mid + 1;
                    } else {
                        let mut start_glyph = read_at(group + 8);
                        if format == Format::Type12 {
                            start_glyph += ch - start_c;
                        }
                        return start_glyph;
                    }
                }
                0
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Format {
    Type0 {
        length: usize,
    },
    Type4 {
        seg_count: usize,
        range_shift: usize,
    },
    Type6 {
        first: usize,
        end: usize,
    },
    Type12,
    Type13,
}