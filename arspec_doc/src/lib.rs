pub mod html;
pub mod svg;

mod font;

pub use self::font::{Font, FontStyle, FontWeight, GlyphIndex};

macro_rules! load_generated_file {
    ($file:tt) => (include!(concat!(env!("OUT_DIR"), $file)));
}

macro_rules! load_manifest_file {
    ($path:tt) => (include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), $path)));
}

pub mod fonts {
    use super::*;

    pub const MONO: Font<'static> = load_generated_file!("/font_mono.rs");
    pub const SANS: Font<'static> = load_generated_file!("/font_sans.rs");
    pub const SANS_BOLD: Font<'static> = load_generated_file!("/font_sans_bold.rs");
    pub const SANS_ITALIC: Font<'static> = load_generated_file!("/font_sans_italic.rs");

    pub const ALL: &'static [&'static Font<'static>] = &[
        &MONO, &SANS, &SANS_BOLD, &SANS_ITALIC
    ];
}

pub mod scripts {
    pub const MAIN: &'static [u8] = load_manifest_file!("/assets/scripts/main.js");
}

pub mod styles {
    pub const PRINT: &'static [u8] = load_manifest_file!("/assets/styles/print.css");
    pub const SCREEN: &'static [u8] = load_manifest_file!("/assets/styles/screen.css");
}
