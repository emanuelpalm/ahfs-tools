mod otf;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

const FONT_MONO: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoMono-Regular-pruned.ttf"));
const FONT_SANS: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Regular-pruned.ttf"));
const FONT_SANS_BOLD: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Bold-pruned.ttf"));
const FONT_SANS_ITALIC: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/noto/NotoSans-Italic-pruned.ttf"));

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

    // Package simpler values.
    {
        let units_per_em = font.head().units_per_em() as f32;
        out.push_str(&format!(
            concat!(
                "Font {{\n",
                "    line_height: {},\n",
                "    units_per_em: {},\n",
            ),
            {
                let hhea = font.hhea();
                (hhea.ascender() - hhea.descender() + hhea.line_gap()) as f32 / units_per_em
            },
            units_per_em,
        ));
    }

    let glyph_indexes: Vec<(u16, u16)> = font.cmap().iter()
        .filter_map(|(ch, glyph_index)| {
            if (ch as u32) < (u16::max_value() as u32) {
                Some((ch as u32 as u16, glyph_index as u16))
            } else {
                None
            }
        })
        .collect();
    let glyph_indexes_len = glyph_indexes.len();

    // Package advance widths.
    {
        let mut advance_widths = Vec::with_capacity(glyph_indexes_len);
        let mut advance_width_index = Vec::with_capacity(glyph_indexes_len);
        {
            let hmtx = font.hmtx();
            for (cp, glyph_index) in &glyph_indexes {
                if let Some(metrics) = hmtx.lookup(*glyph_index as usize) {
                    advance_widths.push(metrics.advance_width);
                    advance_width_index.push(*cp);
                }
            }
        }

        out.push_str("    advance_widths: [");
        let mut offset = 0usize;
        if let Some((last, rest)) = advance_widths.split_last() {
            for advance_width in rest {
                if offset & 0x0f == 0 {
                    out.push_str("\n        ");
                    offset = 0;
                }
                offset += 1;
                out.push_str(&format!("{}, ", advance_width));
            }
            out.push_str(&format!("{}", last));
        }
        out.push_str("],\n    advance_width_index: [");
        offset = 0;
        if let Some((last, rest)) = advance_width_index.split_last() {
            for index in rest {
                if offset & 0x0f == 0 {
                    out.push_str("\n        ");
                    offset = 0;
                }
                offset += 1;
                out.push_str(&format!("{}, ", index));
            }
            out.push_str(&format!("{}", last));
        }
        out.push_str("],\n");
    }

    // Package horizontal kerning values.
    {
        let mut kernings = Vec::new();
        let mut kerning_index = Vec::new();
        if let Some(kern) = font.kern() {
            kernings.reserve(glyph_indexes_len * 4);
            kerning_index.reserve(glyph_indexes_len * 4);
            for (cp_a, glyph_index_a) in &glyph_indexes {
                for (cp_b, glyph_index_b) in &glyph_indexes {
                    let kerning = kern.lookup(*glyph_index_a as usize, *glyph_index_b as usize);
                    if kerning != 0 {
                        kerning_index.push((*cp_a as u32) | ((*cp_b as u32) << 16));
                        kernings.push(kerning);
                    }
                }
            }
        }

        out.push_str("    kernings: [");
        let mut offset = 0usize;
        if let Some((last, rest)) = kernings.split_last() {
            for kerning in rest {
                if offset & 0x0f == 0 {
                    out.push_str("\n        ");
                    offset = 0;
                }
                offset += 1;
                out.push_str(&format!("{}, ", kerning));
            }
            out.push_str(&format!("{}", last));
        }
        out.push_str("],\n    kerning_index: [");
        offset = 0;
        if let Some((last, rest)) = kerning_index.split_last() {
            for index in rest {
                if offset & 0x0f == 0 {
                    out.push_str("\n        ");
                    offset = 0;
                }
                offset += 1;
                out.push_str(&format!("{}, ", index));
            }
            out.push_str(&format!("{}", last));
        }
        out.push_str("],\n");
    }

    out.push('}');

    out_file.write_all(out.as_bytes()).unwrap();
}