#![allow(dead_code)]

use std::char;
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
            _ => { return None; }
        };

        Some(CharacterToGlyphIndexMappingTable {
            subtable,
            format,
        })
    }

    /// Creates new iterator for iterating through all table glyph indexes.
    pub fn iter(&'a self) -> CharacterToGlyphIndexIter<'a> {
        let mut state = CharacterToGlyphIndexIterState::default();
        CharacterToGlyphIndexIter {
            table: self,
            lambda: match self.format {
                Format::Type0 { length } => {
                    state.f0_length = length;
                    |table, state| {
                        if state.offset < state.f0_length {
                            let next = Some((
                                char::from(state.offset as u8),
                                table.subtable.read_u8_at(6 + state.offset).unwrap_or(0) as u32,
                            ));
                            state.offset += 1;
                            next
                        }
                        else {
                            None
                        }
                    }
                },
                Format::Type4 { seg_count, range_shift } => {
                    state.f4_seg_count = seg_count;
                    state.f4_range_shift = range_shift;
                    state.f4_end_codes = 14;
                    state.f4_start_codes = 16 + seg_count * 2;
                    state.f4_id_deltas = 16 + seg_count * 4;
                    state.f4_id_range_offsets = 16 + seg_count * 6;
                    state.f4_end_code = self.subtable.read_u16_at(state.f4_end_codes)
                        .unwrap_or(0xFFFF);
                    state.f4_start_code = self.subtable.read_u16_at(state.f4_start_codes)
                        .unwrap_or(0xFFFF);
                    state.f4_id_delta = self.subtable.read_i16_at(state.f4_id_deltas)
                        .unwrap_or(0);
                    state.f4_id_range_offset = self.subtable.read_u16_at(state.f4_id_range_offsets)
                        .unwrap_or(0);

                    |table, state| {
                        if state.f4_end_code == 0xFFFF {
                            return None;
                        }

                        let next = Some((
                            char::from_u32(state.offset as u32).unwrap_or('�'),
                            match state.f4_id_range_offset {
                                0 => ((state.offset as i32) + (state.f4_id_delta as i32)) as u16,
                                _ => {
                                    let index = state.f4_id_range_offsets
                                        + state.f4_id_range_offset as usize
                                        + ((state.offset - state.f4_start_code as usize) * 2);

                                    let glyph_id = table.subtable.read_u16_at(index)?;
                                    if glyph_id != 0 {
                                        ((glyph_id as i32) + (state.f4_id_delta as i32)) as u16
                                    } else {
                                        0
                                    }
                                }
                            } as u32
                        ));

                        if state.offset == state.f4_end_code as usize {
                            state.f4_end_codes += 2;
                            state.f4_end_code = table.subtable
                                .read_u16_at(state.f4_end_codes)?;

                            state.f4_start_codes += 2;
                            state.f4_start_code = table.subtable
                                .read_u16_at(state.f4_start_codes)?;

                            state.f4_id_deltas += 2;
                            state.f4_id_delta = table.subtable
                                .read_i16_at(state.f4_id_deltas)?;

                            state.f4_id_range_offsets += 2;
                            state.f4_id_range_offset = table.subtable
                                .read_u16_at(state.f4_id_range_offsets)?;

                            state.offset = state.f4_start_code as usize;
                        } else {
                            state.offset += 1;
                        }

                        next
                    }
                },
                Format::Type6 { first, end } => {
                    state.f6_first = first;
                    state.f6_end = end;
                    |table, state| {
                        if state.offset >= state.f6_first && state.offset < state.f6_end {
                            let next = Some((
                                char::from_u32(state.offset as u32).unwrap_or('�'),
                                table.subtable.read_u16_at(10 + (state.offset - state.f6_first) * 2)
                                    .unwrap_or(0) as u32,
                            ));
                            state.offset += 1;
                            next
                        } else {
                            None
                        }
                    }
                },
            },
            state,
        }
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
        }
    }
}

pub struct CharacterToGlyphIndexIter<'a> {
    table: &'a CharacterToGlyphIndexMappingTable<'a>,
    lambda: fn(&CharacterToGlyphIndexMappingTable<'a>, &mut CharacterToGlyphIndexIterState) -> Option<(char, u32)>,
    state: CharacterToGlyphIndexIterState,
}

impl<'a, 'b: 'a> Iterator for CharacterToGlyphIndexIter<'a> {
    type Item = (char, u32);

    #[inline]
    fn next(&mut self) -> Option<(char, u32)> {
        (self.lambda)(self.table, &mut self.state)
    }
}

#[derive(Default)]
struct CharacterToGlyphIndexIterState {
    offset: usize,

    f0_length: usize,

    f4_seg_count: usize,
    f4_range_shift: usize,
    f4_end_codes: usize,
    f4_start_codes: usize,
    f4_id_deltas: usize,
    f4_id_range_offsets: usize,
    f4_end_code: u16,
    f4_start_code: u16,
    f4_id_delta: i16,
    f4_id_range_offset: u16,

    f6_first: usize,
    f6_end: usize,
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
}