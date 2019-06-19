mod otf;

use std::collections::HashMap;
use std::{env, fmt};
use std::fs::File;
use std::path::Path;
use std::io::Write;

const FONT_MONO: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSansMono-Regular-European.ttf"
));
const FONT_SANS: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Regular-European.ttf"
));
const FONT_SANS_BOLD: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Bold-European.ttf"
));
const FONT_SANS_ITALIC: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Italic-European.ttf"
));

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    package_font(FONT_MONO, &Path::new(&out_dir).join("font_mono.rs"));
    package_font(FONT_SANS, &Path::new(&out_dir).join("font_sans.rs"));
    package_font(FONT_SANS_BOLD, &Path::new(&out_dir).join("font_sans_bold.rs"));
    package_font(FONT_SANS_ITALIC, &Path::new(&out_dir).join("font_sans_italic.rs"));
}

fn package_font(font_file: &[u8], dest_path: &Path) {
    let mut out = String::new();
    let mut out_file = File::create(&dest_path).unwrap();

    let font = otf::FontFile::try_new(font_file).unwrap();

    let mut glyphs: Vec<Glyph> = font.cmap()
        .iter()
        .map(|(token, index)| {
            Glyph {
                token,
                index,
                advance_width: 0,
                kerning: Vec::new(),
            }
        })
        .collect();

    {
        let hmtx = font.hmtx();
        for glyph in &mut glyphs {
            if let Some(metrics) = hmtx.lookup(glyph.index) {
                glyph.advance_width = metrics.advance_width;
            }
        }
    }

    let mut glyphs = glyphs.drain(..).fold(vec![Glyph::default(); 127], |mut glyphs, glyph| {
        let token = glyph.token as u32;
        if token < (u16::max_value() as u32) {
            if token < 127 {
                glyphs[token as usize] = glyph;
            } else {
                glyphs.push(glyph);
            }
        }
        glyphs
    });

    let mut indexes = HashMap::new();

    for (index, glyph) in glyphs.iter().enumerate() {
        indexes.insert(glyph.index, index);
    }

    if let Some(kern) = font.kern() {
        for (left, right, kerning) in kern.iter() {
            let left = indexes.get(&(left as u32)).unwrap();
            let right = indexes.get(&(right as u32)).unwrap();
            let glyph = glyphs.get_mut(*left as usize).unwrap();
            glyph.kerning.push((*right as u16, kerning));
        }
    }

    // Package simpler values.
    out.push_str(&format!(
        concat!(
            "Font {{\n",
            "    line_height: {},\n",
            "    units_per_em: {},\n",
        ),
        {
            let hhea = font.hhea();
            hhea.ascender() - hhea.descender() + hhea.line_gap()
        },
        font.head().units_per_em(),
    ));

    // Package non-ASCII glyph indexes.
    out.push_str("    glyph_indexes_nonascii: &");
    package_array(&mut out, &glyphs[127..].iter()
        .map(|g| g.token as u32 as u16)
        .collect::<Vec<u16>>());
    out.push_str(",\n");

    // Package advance widths.
    out.push_str("    advance_widths: &");
    package_array(&mut out, &glyphs.iter()
        .map(|g| g.advance_width)
        .collect::<Vec<u16>>());
    out.push_str(",\n");


    // Package horizontal kerning values.
    if font.kern().is_some() {
        out.push_str("    kernings: Some(&[");
        for glyph in &glyphs {
            out.push_str("\n        &[");
            if let Some(((right, kerning), rest)) = glyph.kerning.split_first() {
                out.push_str(&format!("({}, {})", right, kerning));
                for (right, kerning) in rest {
                    out.push_str(&format!(", ({}, {})", right, kerning));
                }
            }
            out.push_str("],");
        }
        out.push_str("\n    ]),\n");
    } else {
        out.push_str("    kernings: None,\n");
    }

    out.push('}');

    out_file.write_all(out.as_bytes()).unwrap();
}

fn package_array<T: fmt::Display>(out: &mut String, array: &[T]) {
    let mut offset = 0usize;

    out.push('[');
    for item in array {
        if offset & 0x07 == 0 {
            out.push_str("\n       ");
            offset = 0;
        }
        offset += 1;
        out.push_str(&format!(" {},", item));
    }
    if array.len() > 0 {
        out.push_str("\n    ]");
    } else {
        out.push(']');
    }
}

#[derive(Clone, Default)]
struct Glyph {
    token: char,
    index: u32,
    advance_width: u16,
    kerning: Vec<(u16, i16)>,
}