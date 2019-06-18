#![allow(dead_code)]

use super::{IndexToLocationTable, Region};

// Simple glyph flags.
const FLAGS_ON_CURVE_POINT: u8 = 0x01;
const FLAGS_X_SHORT_VECTOR: u8 = 0x02;
const FLAGS_Y_SHORT_VECTOR: u8 = 0x04;
const FLAGS_REPEAT_FLAG: u8 = 0x08;
const FLAGS_X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0x10;
const FLAGS_Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0x20;

// Composite glyph flags.
const FLAGS_ARG_1_AND_2_ARE_WORDS: u16 = 0x0001;
const FLAGS_ARGS_ARE_XY_VALUES: u16 = 0x0002;
const FLAGS_ROUND_XY_TO_GRID: u16 = 0x0004;
const FLAGS_WE_HAVE_A_SCALE: u16 = 0x0008;
const FLAGS_MORE_COMPONENTS: u16 = 0x0020;
const FLAGS_WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;
const FLAGS_WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080;
const FLAGS_WE_HAVE_INSTRUCTIONS: u16 = 0x0100;
const FLAGS_USE_MY_METRICS: u16 = 0x0200;
const FLAGS_OVERLAP_COMPOUND: u16 = 0x0400;
const FLAGS_SCALED_COMPONENT_OFFSET: u16 = 0x0800;
const FLAGS_UNSCALED_COMPONENT_OFFSET: u16 = 0x1000;

/// This table contains information that describes the glyphs in the font in
/// the TrueType outline format.
pub struct GlyphDataTable<'a> {
    region: Region<'a>,
    loca: IndexToLocationTable<'a>,
}

impl<'a> GlyphDataTable<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn new(glyf: Region<'a>, loca: IndexToLocationTable<'a>) -> Self {
        GlyphDataTable { region: glyf, loca }
    }

    /// Looks up contour points of identified glyph.
    pub fn lookup(&self, glyph_index: usize) -> Option<Vec<Point>> {
        let glyph_range = self.loca.lookup(glyph_index)?;
        let glyph = self.region.subregion(glyph_range)?;
        let number_of_contours = glyph.read_i16_at(0)?;
        match number_of_contours {
            n @ _ if n > 0 => self.lookup_simple(glyph, n as usize),
            n @ _ if n < 0 => self.lookup_composite(glyph),
            _ => None,
        }
    }

    #[inline]
    fn lookup_simple(&self, glyph: Region<'a>, number_of_contours: usize) -> Option<Vec<Point>> {
        let mut offset = 10;

        let mut end_pts_of_contours = vec![0; number_of_contours];
        if !glyph.read_u16s_at(offset, &mut end_pts_of_contours) {
            return None;
        }

        // Skip instructions.
        {
            offset += number_of_contours as usize * 2;
            let instruction_length = glyph.read_u16_at(offset)?;
            offset += instruction_length as usize;
        }

        let number_of_points = end_pts_of_contours[number_of_contours - 1] as usize + 1;

        let mut flags_all = Vec::with_capacity(number_of_points);
        let mut points = Vec::with_capacity(number_of_points);

        // Collect point flags.
        {
            let mut repeat = 0;
            let mut flags = 0;
            for _ in 0..number_of_points {
                if repeat == 0 {
                    flags = glyph.read_u8_at(offset)?;
                    offset += 1;
                    if flags & FLAGS_REPEAT_FLAG != 0 {
                        repeat = glyph.read_u8_at(offset)?;
                        offset += 1;
                    }
                } else {
                    repeat -= 1;
                }
                flags_all.push(flags);
                points.push(Point {
                    on_curve: flags & FLAGS_ON_CURVE_POINT != 0,
                    contour_end: false,
                    x: 0,
                    y: 0,
                });
            }
        }


        // Collect point x coordinates.
        {
            let mut x = 0;
            for (index, flags) in flags_all.iter().enumerate() {
                if flags & FLAGS_X_SHORT_VECTOR != 0 {
                    x = glyph.read_u8_at(offset)? as i16;
                    offset += 1;
                    if flags & FLAGS_X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR == 0 {
                        x = -x;
                    }
                } else if flags & FLAGS_X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR == 0 {
                    x = glyph.read_i16_at(offset)?;
                    offset += 2;
                }
                points[index].x = x;
            }
        }

        // Collect point y coordinates.
        {
            let mut y = 0;
            for (index, flags) in flags_all.iter().enumerate() {
                if flags & FLAGS_Y_SHORT_VECTOR != 0 {
                    y = glyph.read_u8_at(offset)? as i16;
                    offset += 1;
                    if flags & FLAGS_Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR == 0 {
                        y = -y;
                    }
                } else if flags & FLAGS_Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR == 0 {
                    y = glyph.read_i16_at(offset)?;
                    offset += 2;
                }
                points[index].y = y;
            }
        }

        // Collect contour ends.
        {
            for end_pts in end_pts_of_contours {
                points[end_pts as usize].contour_end = true;
            }
        }

        Some(points)
    }

    #[inline]
    fn lookup_composite(&self, glyph: Region<'a>) -> Option<Vec<Point>> {
        let mut points = Vec::new();
        let mut offset = 10;
        loop {
            // Collect flags and glyph index.
            let (flags, glyph_index) = glyph.read_2x_u16_at(offset)?;
            offset += 4;

            // Collect arguments.
            let (x, y);
            if flags & FLAGS_ARGS_ARE_XY_VALUES != 0 {
                let (a, b) = if flags & FLAGS_ARG_1_AND_2_ARE_WORDS != 0 {
                    let (a, b) = glyph.read_2x_i16_at(offset)?;
                    offset += 4;
                    (a, b)
                } else {
                    let (a, b) = glyph.read_2x_u8_at(offset)?;
                    offset += 2;
                    (a as i16, b as i16)
                };
                x = a as f32;
                y = b as f32;
            } else {
                // Matching points not supported.
                return None;
            }

            // Assemble 2x2 transformation matrix.
            let mut matrix = (1.0, 0.0, 0.0, 1.0);
            if flags & FLAGS_WE_HAVE_A_SCALE != 0 {
                matrix.0 = glyph.read_i16_at(offset)? as f32 / 16384.0;
                offset += 2;
                matrix.3 = matrix.0;
            } else if flags & FLAGS_WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                let (a, b) = glyph.read_2x_i16_at(offset)?;
                offset += 4;
                matrix.0 = a as f32 / 16384.0;
                matrix.3 = b as f32 / 16384.0;
            } else if flags & FLAGS_WE_HAVE_A_TWO_BY_TWO != 0 {
                let (a, b) = glyph.read_2x_i16_at(offset)?;
                offset += 4;
                let (c, d) = glyph.read_2x_i16_at(offset)?;
                offset += 4;
                matrix.0 = a as f32 / 16384.0;
                matrix.1 = b as f32 / 16384.0;
                matrix.2 = c as f32 / 16384.0;
                matrix.3 = d as f32 / 16384.0;
            }

            // Lookup referenced glyph.
            let points0 = self.lookup(glyph_index as usize).unwrap_or_else(Vec::new);

            // Collect and transform glyph points.
            if !points0.is_empty() {
                let m = (matrix.0 * matrix.0 + matrix.1 * matrix.1).sqrt();
                let n = (matrix.2 * matrix.2 + matrix.3 * matrix.3).sqrt();
                for mut point in points0 {
                    let (a, b) = (point.x as f32, point.y as f32);
                    point.x = (m * (matrix.0 * a + matrix.2 * b + x)) as i16;
                    point.y = (n * (matrix.1 * a + matrix.3 * b + y)) as i16;
                    points.push(point);
                }
            }

            if flags & FLAGS_MORE_COMPONENTS == 0 {
                break;
            }
        }
        Some(points)
    }
}

/// A TrueType outline point.
///
/// Sequences of these points are used to describe the contours of glyphs,
/// where a contour is a distinct shape part of the glyph. All coordinates are
/// relative to the previous ones. If there is no previous point, (0, 0) is
/// assumed.
pub struct Point {
    /// Whether or not this point is on the current contour curve.
    pub on_curve: bool,

    /// Whether or not this point is the last in its contour.
    pub contour_end: bool,

    /// X-coordinate relative to last point.
    pub x: i16,

    /// Y-coordinate relative to last point.
    pub y: i16,
}
