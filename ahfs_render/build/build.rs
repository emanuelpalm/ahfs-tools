mod font;

use crate::font::otf::*;
use std::collections::BTreeSet;
use std::fs;

fn main() {
    let font = fs::read("/usr/share/fonts/truetype/croscore/Arimo-Regular.ttf").unwrap();
    let font = FontFile::try_from(&font).unwrap();

    println!("{:?}", font.v_metrics());

    for (index, hmetrics) in font.h_metrics_iter().enumerate() {
        println!("{:05}: {:?}", index, hmetrics);
    }

    panic!();

    /*
    let upe = font.units_per_em();

    let mut glyphs = BTreeSet::new();
    for i in 0..65828u32 {
        let glyph = font.find_glyph_index(i);
        if glyph > 0 {
            glyphs.insert(glyph);
        }
    }

    let mut aws = Vec::new();
    let mut aw_last = 0;
    for glyph in &glyphs {
        let aw = font.get_glyph_h_metrics(*glyph).advance_width;
        if aw != aw_last {
            aws.push((*glyph, aw));
            aw_last = aw;
        }
    }

    println!("Advance widths: {}", aws.len());

    let mut kas = Vec::new();
    let mut ka_last = 0;
    for y in &glyphs {
        for x in &glyphs {
            let ka = font.get_glyph_kern_advance(*x, *y);
            if ka != 0 && ka != ka_last {
                kas.push((*x, *y, ka));
                ka_last = ka;
            }
        }
    }

    println!("Kernings: {}", kas.len());*/
}