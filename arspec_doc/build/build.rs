mod otf;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
/// Generates `Font` from referenced `font_file` and writes it to `dest_path`.
fn build_font<'a>(font: &otf::FontFile<'a>, file_name: &str, name: &str, dest: &Path) {
    // Prepare font glyphs.
    let glyphs = {
        // Create one glyph per font glyph index.
        let mut glyphs: Vec<Glyph> = font.cmap()
            .iter()
            .map(|(token, index)| Glyph { token, index, ..Glyph::default() })
            .collect();

        // Resolve advance widths for all glyphs.
        {
            let hmtx = font.hmtx();
            for glyph in &mut glyphs {
                if let Some(metrics) = hmtx.lookup(glyph.index) {
                    glyph.advance_width = metrics.advance_width;
                }
            }
        }

        // Create glyphs for all missing ASCII characters.
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

        // Resolve kerning values for all glyphs, if any.
        if let Some(kern) = font.kern() {
            // Map between old and new glyph indexes.
            let mut indexes = HashMap::new();
            for (index, glyph) in glyphs.iter().enumerate() {
                indexes.insert(glyph.index, index);
            }
            // Create kerning lists for all glyphs.
            for (left, right, kerning) in kern.iter() {
                let left = indexes.get(&(left as u32)).unwrap();
                let right = indexes.get(&(right as u32)).unwrap();
                let glyph = glyphs.get_mut(*left as usize).unwrap();
                glyph.kerning.push((*right as u16, kerning));
            }
        }

        glyphs
    };

    // Render font glyphs.
    let out = format!(
        concat!(
            "Font {{\n",
            "    name: \"{name}\",\n",
            "    style: FontStyle::{style},\n",
            "    weight: FontWeight::{weight},\n",
            "\n",
            "    ascender: {ascender},\n",
            "    descender: {descender},\n",
            "    line_gap: {line_gap},\n",
            "    units_per_em: {units_per_em},\n",
            "\n",
            "    glyph_indexes_nonascii: {glyph_indexes_nonascii},\n",
            "    advance_widths: {advance_widths},\n",
            "    kernings: {kernings},\n",
            "\n",
            "    source: {source},\n",
            "    source_name: \"{source_name}\",\n",
            "}}\n"
        ),
        name = name,
        style = if font.head().mac_style_italic() { "Italic" } else { "Normal" },
        weight = if font.head().mac_style_bold() { "Bold" } else { "Normal" },
        ascender = font.hhea().ascender(),
        descender = font.hhea().descender(),
        line_gap = font.hhea().line_gap(),
        units_per_em = font.head().units_per_em(),
        glyph_indexes_nonascii = {
            let mut out = "&[".to_string();
            let mut column = 8;
            for glyph in &glyphs[127..] {
                if column == 8 {
                    out.push_str("\n        ");
                    column = 0;
                }
                column += 1;
                out.push_str(&format!("{}, ", glyph.token as u32 as u16));
            }
            out.push_str("\n    ]");
            out
        },
        advance_widths = {
            let mut out = "&[".to_string();
            let mut column = 8;
            for glyph in &glyphs {
                if column == 8 {
                    out.push_str("\n        ");
                    column = 0;
                }
                column += 1;
                out.push_str(&format!("{}, ", glyph.advance_width));
            }
            out.push_str("\n    ]");
            out
        },
        kernings = {
            let mut out = String::new();
            if font.kern().is_some() {
                out.push_str("Some(&[");
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
                out.push_str("\n    ])");
            } else {
                out.push_str("None");
            }
            out
        },
        source = {
            let mut out = "&[".to_string();
            let mut column = 14;
            for b in font.as_bytes().iter() {
                if column == 14 {
                    out.push_str("\n        ");
                    column = 0;
                }
                column += 1;
                out.push_str(&format!("{:3}, ", *b));
            }
            out.push_str("\n    ]");
            out
        },
        source_name = file_name,
    );

    // Write font glyphs to new file at destination path.
    File::create(&dest)
        .unwrap()
        .write_all(out.as_bytes())
        .unwrap();
}

use std::io::Write;

macro_rules! load_font_file {
    ($path:tt) => {
        $crate::otf::FontFile::try_new(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/", $path))
        ).unwrap()
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    build_font(
        &load_font_file!("NotoSansMono-Regular-European.ttf"),
        "NotoSansMono-Regular-European.ttf",
        "Noto Sans Mono",
        &out_dir.join("font_mono.rs"),
    );

    build_font(
        &load_font_file!("NotoSans-Regular-European.ttf"),
        "NotoSans-Regular-European.ttf",
        "Noto Sans",
        &out_dir.join("font_sans.rs"),
    );

    build_font(
        &load_font_file!("NotoSans-Bold-European.ttf"),
        "NotoSans-Bold-European.ttf",
        "Noto Sans",
        &out_dir.join("font_sans_bold.rs"),
    );

    build_font(
        &load_font_file!("NotoSans-Italic-European.ttf"),
        "NotoSans-Italic-European.ttf",
        "Noto Sans",
        &out_dir.join("font_sans_italic.rs"),
    );
}

#[derive(Clone, Default)]
struct Glyph {
    token: char,
    index: u32,
    advance_width: u16,
    kerning: Vec<(u16, i16)>,
}