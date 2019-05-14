use std::char;
use std::ops;
use std::ptr;
use std::slice;

const PLATFORM_UNICODE: u16 = 0;
const PLATFORM_MACINTOSH: u16 = 1;
const PLATFORM_WINDOWS: u16 = 3;

const PLATFORM_WINDOWS_UNICODE_BMP: u16 = 1;
const PLATFORM_WINDOWS_UNICODE_FULL: u16 = 10;

pub struct FontFile<'a> {
    data: Block<'a>,
    hhea: Block<'a>,
    hmtx: Block<'a>,
    kern: Option<Block<'a>>,

    encoding: Block<'a>,
    encoding_format: EncodingFormat,

    glyph_count: u16,
}

impl<'a> FontFile<'a> {
    pub fn try_from(data: &'a [u8]) -> Option<Self> {
        let data = Block::new(data);

        // Locate OTF tables.
        let mut cmap = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut maxp = None;
        {
            let table_count = data.read_u16_at(4)? as usize;
            for i in 0..table_count {
                let offset = 12 + 16 * i;
                let target = match data.get(offset..offset + 4)? {
                    b"cmap" => &mut cmap,
                    b"hhea" => &mut hhea,
                    b"hmtx" => &mut hmtx,
                    b"kern" => &mut kern,
                    b"maxp" => &mut maxp,
                    _ => continue,
                };
                let from = data.read_u32_at(offset + 8)? as usize;
                let to = from + data.read_u32_at(offset + 12)? as usize;
                *target = data.subsection(from..to);
            }
        }
        let cmap = cmap?;

        // Locate a supported encoding record.
        let mut encoding = None;
        {
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
                encoding = data.subsection(cmap.range.start + index..data.data.len());
                break;
            }
        }
        let encoding = encoding?;
        let encoding_format = EncodingFormat::try_from(encoding.read_u16_at(0)?)?;

        Some(FontFile {
            data,
            hhea: hhea?,
            hmtx: hmtx?,
            kern,
            encoding,
            encoding_format,
            glyph_count: maxp?.read_u16_at(4)?,
        })
    }

    pub fn char_glyph_iter<'b: 'a>(&'a self) -> CharGlyphIter<'b, 'a> {
        return CharGlyphIter {
            encoding: &self.encoding,
            lambda: match self.encoding_format {
                EncodingFormat::Format0 => {
                    let length = self.encoding.read_u16_at(2)
                        .map(|length| length.saturating_sub(6) as usize)
                        .unwrap_or(0);

                    let mut index: usize = 0;
                    Box::new(move |encoding: &Block| if index < length {
                        let ch = char::from(index as u8);
                        let glyph_id = encoding.read_u8_at(6 + index)? as u32;
                        index += 1;
                        Some((ch, glyph_id))
                    } else {
                        None
                    })
                }

                EncodingFormat::Format4 => {
                    let segment_count = self.encoding.read_u16_at(6)
                        .map(|segment_count| (segment_count / 2) as usize)
                        .unwrap_or(0);

                    let mut end_codes = 14;
                    let mut start_codes = 16 + segment_count * 2;
                    let mut id_deltas = 16 + segment_count * 4;
                    let mut id_range_offsets = 16 + segment_count * 6;

                    let mut end_code = self.encoding.read_u16_at(end_codes).unwrap_or(0xFFFF);
                    let mut start_code = self.encoding.read_u16_at(start_codes).unwrap_or(0xFFFF);
                    let mut id_delta = self.encoding.read_i16_at(id_deltas).unwrap_or(0);
                    let mut id_range_offset = self.encoding.read_u16_at(id_range_offsets).unwrap_or(0);

                    let mut c = start_code;

                    Box::new(move |encoding: &Block| {
                        if end_code == 0xFFFF {
                            return None;
                        }

                        let next = Some((
                            char::from_u32(c as u32).unwrap_or('�'),
                            match id_range_offset {
                                0 => ((c as i32) + (id_delta as i32)) as u16,
                                _ => {
                                    let index = id_range_offsets
                                        + id_range_offset as usize
                                        + ((c - start_code) * 2) as usize;

                                    let mut glyph_id = encoding.read_u16_at(index)?;
                                    if glyph_id != 0 {
                                        ((glyph_id as i32) + (id_delta as i32)) as u16
                                    } else {
                                        0
                                    }
                                }
                            } as u32
                        ));

                        if c == end_code {
                            end_codes += 2;
                            end_code = encoding.read_u16_at(end_codes)?;

                            start_codes += 2;
                            start_code = encoding.read_u16_at(start_codes)?;

                            id_deltas += 2;
                            id_delta = encoding.read_i16_at(id_deltas)?;

                            id_range_offsets += 2;
                            id_range_offset = encoding.read_u16_at(id_range_offsets)?;

                            c = start_code;
                        } else {
                            c += 1;
                        }

                        return next;
                    })
                }

                EncodingFormat::Format6 => {
                    let mut code = self.encoding.read_u16_at(8).unwrap_or(0xFFFF);
                    let entry_count = self.encoding.read_u16_at(10).unwrap_or(0);
                    let mut entry_index = 0;
                    let mut index = 12;

                    Box::new(move |encoding: &Block| {
                        if entry_index == entry_count {
                            return None;
                        }

                        let next = Some((
                            char::from_u32(code as u32).unwrap_or('�'),
                            encoding.read_u16_at(index)? as u32,
                        ));

                        code += 1;
                        entry_index += 1;
                        index += 2;

                        return next;
                    })
                }
            },
        };
    }

    pub fn h_metrics_iter<'b: 'a>(&'a self) -> HMetricsIter<'b, 'a> {
        HMetricsIter {
            file: &self,
            long_hmetrics_count: self.hhea.read_u16_at(34).unwrap_or(0),
            index: 0,
        }
    }

    pub fn v_metrics(&self) -> VMetrics {
        let mut metrics = self.hhea.i16s().skip(2);
        unsafe {
            VMetrics {
                ascent: metrics.next().unwrap_or(0),
                descent: metrics.next().unwrap_or(0),
                line_gap: metrics.next().unwrap_or(0),
            }
        }
    }
}

pub struct CharGlyphIter<'a, 'b: 'a> {
    encoding: &'a Block<'b>,
    lambda: Box<FnMut(&Block<'b>) -> Option<(char, u32)>>,
}

impl<'a, 'b: 'a> Iterator for CharGlyphIter<'a, 'b> {
    type Item = (char, u32);

    #[inline]
    fn next(&mut self) -> Option<(char, u32)> {
        (self.lambda)(self.encoding)
    }
}

enum EncodingFormat {
    Format0,
    Format4,
    Format6,
}

impl EncodingFormat {
    fn try_from(id: u16) -> Option<Self> {
        Some(match id {
            0 => EncodingFormat::Format0,
            4 => EncodingFormat::Format4,
            6 => EncodingFormat::Format6,
            _ => { return None; }
        })
    }
}

#[derive(Debug)]
pub struct HMetrics {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

pub struct HMetricsIter<'a, 'b: 'a> {
    file: &'a FontFile<'b>,
    long_hmetrics_count: u16,
    index: usize,
}

impl<'a, 'b: 'a> Iterator for HMetricsIter<'a, 'b> {
    type Item = HMetrics;

    fn next(&mut self) -> Option<HMetrics> {
        let index = self.index;
        if index >= (self.file.glyph_count as usize) {
            return None;
        }
        self.index += 1;

        let hmtx = &self.file.hmtx;

        if index < (self.long_hmetrics_count as usize) {
            let offset = 4 * index;
            Some(HMetrics {
                advance_width: hmtx.read_u16_at(offset)?,
                left_side_bearing: hmtx.read_i16_at(offset + 2)?,
            })
        } else {
            let aw_offset = 4 * (self.long_hmetrics_count - 1) as usize;
            let lsb_offset = (4 * self.long_hmetrics_count) as usize
                + (2 * (index as isize - self.long_hmetrics_count as isize))
                as usize;

            Some(HMetrics {
                advance_width: hmtx.read_u16_at(aw_offset)?,
                left_side_bearing: hmtx.read_i16_at(lsb_offset)?,
            })
        }
    }
}

pub struct Block<'a> {
    range: ops::Range<usize>,
    data: &'a [u8],
}

macro_rules! be_read_at {
    (fn $name:ident(&self, ...) -> $typ:ident) => {
        fn $name(&self, offset: usize) -> Option<$typ> {
            self.data
                .get(offset..offset + std::mem::size_of::<$typ>())
                .map(|r| $typ::from_be(unsafe { (r.as_ptr() as *const $typ).read() }))
        }
    };
}

macro_rules! be_iter {
    (fn $name:ident(&self) -> $typ_impl:ident<$lifetime:lifetime, _>) => {
        #[inline]
        pub fn $name<'b: $lifetime>(&'b self) -> $typ_impl<$lifetime, 'b> {
            $typ_impl {
                block: self,
                index: 0,
            }
        }
    };

    (impl Iterator<Item=$typ:ty> for $name:ident using $section_read:ident) => {
        pub struct $name<'a, 'b: 'a> {
            block: &'a Block<'b>,
            index: usize,
        }

        impl<'a, 'b: 'a> $name<'a, 'b> {
            #[allow(dead_code)]
            #[inline]
            pub fn skip(mut self, n: usize) -> Self {
                self.index += n * std::mem::size_of::<$typ>();
                self
            }
        }

        impl<'a, 'b: 'a> Iterator for $name<'a, 'b> {
            type Item = $typ;

            fn next(&mut self) -> Option<$typ> {
                if self.index < self.block.len() {
                    let item = self.block.$section_read(self.index);
                    self.index += std::mem::size_of::<$typ>();
                    return item;
                }
                None
            }
        }
    };
}

impl<'a> Block<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Block { range: 0..data.len(), data }
    }

    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where I: slice::SliceIndex<[u8]>
    {
        self.data.get(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    be_read_at!(fn read_i8_at(&self, ...) -> i8);
    be_read_at!(fn read_i16_at(&self, ...) -> i16);
    be_read_at!(fn read_i32_at(&self, ...) -> i32);
    be_read_at!(fn read_u8_at(&self, ...) -> u8);
    be_read_at!(fn read_u16_at(&self, ...) -> u16);
    be_read_at!(fn read_u32_at(&self, ...) -> u32);

    pub fn subsection(&self, range: ops::Range<usize>) -> Option<Block<'a>> {
        if range.start > (isize::max_value() as usize)
            || range.end > (isize::max_value() as usize)
            || range.start > range.end
            || range.end > self.data.len()
        {
            return None;
        }
        unsafe {
            let start = self.data.as_ptr().offset(range.start as isize);
            Some(Block {
                range: range.clone(),
                data: slice::from_raw_parts(start, range.end - range.start),
            })
        }
    }

    be_iter!(fn i16s(&self) -> I16s<'a, _>);
    be_iter!(fn u16s(&self) -> U16s<'a, _>);
}

be_iter!(impl Iterator<Item=i16> for I16s using read_i16_at);
be_iter!(impl Iterator<Item=u16> for U16s using read_u16_at);

#[derive(Debug)]
pub struct VMetrics {
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
}

impl VMetrics {
    #[inline]
    pub fn advance_height(&self) -> i16 {
        self.ascent - self.descent + self.line_gap
    }
}