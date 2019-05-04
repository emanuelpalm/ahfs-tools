use std::ops;
use std::ptr;
use std::slice;

const PLATFORM_UNICODE: u16 = 0;
const PLATFORM_MACINTOSH: u16 = 1;
const PLATFORM_WINDOWS: u16 = 3;

const PLATFORM_WINDOWS_UNICODE_BMP: u16 = 1;
const PLATFORM_WINDOWS_UNICODE_FULL: u16 = 10;

pub struct FontFile<'a> {
    data: Section<'a>,
    hhea: Section<'a>,
    hmtx: Section<'a>,
    kern: Option<Section<'a>>,

    encoding: Section<'a>,
    cmap_get: fn(&FontFile<'a>, char) -> u32,

    glyph_count: u16,
}

impl<'a> FontFile<'a> {
    pub fn try_from(data: &'a [u8]) -> Option<Self> {
        let data = Section::new(data);

        // Locate OTF tables.
        let mut cmap = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut maxp = None;
        {
            let table_count = data.read_u16_at(4) as usize;
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
                let from = data.read_u32_at(offset + 8) as usize;
                let to = from + data.read_u32_at(offset + 12) as usize;
                *target = data.subsection(from..to);
            }
        }
        let cmap = cmap?;

        // Locate a suitable encoding record.
        let mut encoding = None;
        {
            let table_count = cmap.read_u16_at(2) as usize;
            for i in 0..table_count {
                let offset = 4 + 8 * i;
                match cmap.read_u16_at(offset) {
                    PLATFORM_UNICODE => {},
                    PLATFORM_WINDOWS => {
                        match cmap.read_u16_at(offset + 2) {
                            PLATFORM_WINDOWS_UNICODE_BMP |
                            PLATFORM_WINDOWS_UNICODE_FULL => {},
                            _ => continue,
                        }
                    },
                    _ => continue,
                }
                let index = cmap.read_u32_at(offset + 4) as usize;
                encoding = cmap.subsection(index..cmap.data.len());
                break;
            }
        }
        let encoding = encoding?;

        let fid = encoding.read_u16_at(0);
        let cmap_get = match fid {
            0 => format0,
            4 => format4,
            _ => format0,
        };

        println!("fid: {}", fid);

        fn format0(font: &FontFile, ch: char) -> u32 {
            let length = font.encoding.read_u16_at(2) as u32 - 6;
            if (ch as u32) < length {
                return font.encoding.read_u8_at(6 + ch as usize) as u32;
            }
            0
        }

        fn format4(font: &FontFile, ch: char) -> u32 {
            let ch = ch as u32;
            let encoding = &font.encoding;

            // standard mapping for windows fonts: binary search collection of ranges
            let segcount = encoding.read_u16_at(6) as usize >> 1;
            let mut search_range = encoding.read_u16_at(8) as usize >> 1;
            let mut entry_selector = encoding.read_u16_at(8);
            let range_shift = encoding.read_u16_at(12) as usize >> 1;

            // do a binary search of the segments
            let end_count = 14;
            let mut search = end_count;

            if ch > 0xffff {
                return 0;
            }

            // they lie from endCount .. endCount + segCount
            // but searchRange is the nearest power of two, so...
            if ch >= encoding.read_u16_at(search + range_shift * 2) as u32 {
                search += range_shift * 2;
            }

            // now decrement to bias correctly to find smallest
            search -= 2;
            while entry_selector != 0 {
                search_range >>= 1;
                let end = encoding.read_u16_at(search + search_range * 2) as u32;
                if ch > end {
                    search += search_range * 2;
                }
                entry_selector -= 1;
            }
            search += 2;

            {
                let item = (search - end_count) >> 1;
                if ch > encoding.read_u16_at(end_count + 2 * item) as u32 {
                    return 0;
                }
                let start = encoding.read_u16_at(14 + segcount * 2 + 2 + 2 * item) as u32;
                if ch < start {
                    return 0;
                }
                let offset = encoding.read_u16_at(14 + segcount * 6 + 2 + 2 * item) as usize;
                if offset == 0 {
                    return (ch as i32
                        + encoding.read_u16_at(14 + segcount * 4 + 2 + 2 * item) as i32)
                        as u16 as u32;
                }
                encoding.read_u16_at(
                    offset
                        + (ch - start) as usize * 2
                        + 14
                        + segcount * 6
                        + 2
                        + 2 * item,
                ) as u32
            }
        }

        return Some(FontFile {
            data,
            hhea: hhea?,
            hmtx: hmtx?,
            kern,
            encoding,
            cmap_get,
            glyph_count: maxp?.read_u16_at(4),
        });
    }

    pub fn char_glyph_iter<'b: 'a>(&'a self) -> CharGlyphIter<'b, 'a> {
        CharGlyphIter {
            font: &self,
            index: 0,
        }
    }

    pub fn h_metrics_iter<'b: 'a>(&'a self) -> HMetricsIter<'b, 'a> {
        HMetricsIter {
            font: &self,
            long_hmetrics_count: self.hhea.read_u16_at(34),
            index: 0,
        }
    }

    pub fn v_metrics(&self) -> VMetrics {
        let mut metrics = [0i16; 3];
        self.hhea.read_i16s_at(4, &mut metrics);
        unsafe {
            VMetrics {
                ascent: *metrics.get_unchecked(0),
                descent: *metrics.get_unchecked(1),
                line_gap: *metrics.get_unchecked(2),
            }
        }
    }
}

pub struct CharGlyphIter<'a, 'b: 'a> {
    font: &'a FontFile<'b>,
    index: usize,
}

#[derive(Debug)]
pub struct HMetrics {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

pub struct HMetricsIter<'a, 'b: 'a> {
    font: &'a FontFile<'b>,
    long_hmetrics_count: u16,
    index: usize,
}

impl<'a, 'b: 'a> Iterator for HMetricsIter<'a, 'b> {
    type Item = HMetrics;

    fn next(&mut self) -> Option<HMetrics> {
        let index = self.index;
        if index >= (self.font.glyph_count as usize) {
            return None;
        }
        self.index += 1;

        let hmtx = &self.font.hmtx;

        if index < (self.long_hmetrics_count as usize) {
            let offset = 4 * index;
            Some(HMetrics {
                advance_width: hmtx.read_u16_at(offset),
                left_side_bearing: hmtx.read_i16_at(offset + 2),
            })
        } else {
            let aw_offset = 4 * (self.long_hmetrics_count - 1) as usize;
            let lsb_offset = (4 * self.long_hmetrics_count) as usize
                + (2 * (index as isize - self.long_hmetrics_count as isize))
                as usize;

            Some(HMetrics {
                advance_width: hmtx.read_u16_at(aw_offset),
                left_side_bearing: hmtx.read_i16_at(lsb_offset),
            })
        }
    }
}

pub struct Section<'a> {
    data: &'a [u8],
}

macro_rules! gen_read_at {
    ($name:ident, $typ:ident) => {
        fn $name(&self, offset: usize) -> $typ {
            self.data
                .get(offset..offset + std::mem::size_of::<$typ>())
                .map(|r| $typ::from_be(unsafe { (r.as_ptr() as *const $typ).read() }))
                .unwrap_or(0)
        }
    };

    ($name:ident, [$typ:ident]) => {
        fn $name(&self, offset: usize, target: &mut [$typ]) {
            let bytes_len = target.len() * std::mem::size_of::<$typ>();
            if let Some(mut bytes) = self.data.get(offset..offset + bytes_len) {
                unsafe {
                    ptr::copy_nonoverlapping(
                        bytes.as_ptr() as *const $typ,
                        target.as_mut_ptr(),
                        target.len(),
                    );
                }
                for i in target {
                    *i = $typ::from_be(*i);
                }
            }
        }
    };
}

impl<'a> Section<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Section { data }
    }

    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where I: slice::SliceIndex<[u8]>
    {
        self.data.get(index)
    }

    gen_read_at!(read_i8_at, i8);
    gen_read_at!(read_i8s_at, [i8]);
    gen_read_at!(read_i16_at, i16);
    gen_read_at!(read_i16s_at, [i16]);
    gen_read_at!(read_i32_at, i32);
    gen_read_at!(read_i32s_at, [i32]);
    gen_read_at!(read_u8_at, u8);
    gen_read_at!(read_u8s_at, [u8]);
    gen_read_at!(read_u16_at, u16);
    gen_read_at!(read_u16s_at, [u16]);
    gen_read_at!(read_u32_at, u32);
    gen_read_at!(read_u32s_at, [u32]);

    pub fn subsection(&self, range: ops::Range<usize>) -> Option<Section<'a>> {
        if range.start > (isize::max_value() as usize)
            || range.end > (isize::max_value() as usize)
            || range.start > range.end
            || range.end > self.data.len()
        {
            return None;
        }
        unsafe {
            let start = self.data.as_ptr().offset(range.start as isize);
            Some(Section {
                data: slice::from_raw_parts(start, range.end - range.start),
            })
        }
    }
}

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