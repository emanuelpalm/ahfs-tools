use std::{ops, slice};

/// A byte region within an OTF font file.
pub struct Region<'a> {
    range: ops::Range<usize>,
    bytes: &'a [u8],
}

macro_rules! be_read_at {
    (fn $name:ident(&self, ...) -> $typ:ident) => {
        #[inline]
        pub fn $name(&self, offset: usize) -> Option<$typ> {
            self.get(offset..offset + std::mem::size_of::<$typ>())
                .map(|r| $typ::from_be(unsafe { (r.as_ptr() as *const $typ).read() }))
        }
    };
}

macro_rules! be_read_2x_at {
    (fn $name:ident(&self, ...) -> $typ:ident) => {
        #[inline]
        pub fn $name(&self, offset: usize) -> Option<($typ, $typ)> {
            let size = std::mem::size_of::<$typ>() * 2;
            self.get(offset..offset + size).map(|r| {
                let ptr = r.as_ptr() as *const $typ;
                unsafe {
                    (
                        $typ::from_be(ptr.read()),
                        $typ::from_be(ptr.offset(1).read()),
                    )
                }
            })
        }
    };
}

macro_rules! be_read_n_at {
    (fn $name:ident(&self, [$typ:ident]) -> bool) => {
        #[inline]
        pub fn $name(&self, offset: usize, target: &mut [$typ]) -> bool {
            let size = std::mem::size_of::<$typ>() * target.len();
            return self.get(offset..offset + size)
                .map(|r| {
                    let ptr = r.as_ptr() as *const $typ;
                    unsafe {
                        ptr.copy_to_nonoverlapping(target.as_mut_ptr(), target.len());
                    }
                    true
                })
                .unwrap_or(false)
        }
    };
}

impl<'a> Region<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Region { range: 0..bytes.len(), bytes }
    }

    #[inline]
    pub fn range(&self) -> &ops::Range<usize> {
        &self.range
    }

    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where I: slice::SliceIndex<[u8]>
    {
        self.bytes.get(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    be_read_at!(fn read_i16_at(&self, ...) -> i16);
    be_read_at!(fn read_i64_at(&self, ...) -> i64);
    be_read_at!(fn read_u8_at(&self, ...) -> u8);
    be_read_at!(fn read_u16_at(&self, ...) -> u16);
    be_read_at!(fn read_u32_at(&self, ...) -> u32);

    be_read_2x_at!(fn read_2x_i16_at(&self, ...) -> i16);
    be_read_2x_at!(fn read_2x_u8_at(&self, ...) -> u8);
    be_read_2x_at!(fn read_2x_u16_at(&self, ...) -> u16);
    be_read_2x_at!(fn read_2x_u32_at(&self, ...) -> u32);

    be_read_n_at!(fn read_u16s_at(&self, [u16]) -> bool);

    pub fn subregion(&self, range: ops::Range<usize>) -> Option<Region<'a>> {
        if range.start > (isize::max_value() as usize)
            || range.end > (isize::max_value() as usize)
            || range.start > range.end
            || range.end > self.bytes.len()
        {
            return None;
        }
        unsafe {
            let start = self.bytes.as_ptr().offset(range.start as isize);
            Some(Region {
                range: range.clone(),
                bytes: slice::from_raw_parts(start, range.end - range.start),
            })
        }
    }
}
